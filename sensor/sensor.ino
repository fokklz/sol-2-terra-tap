#include <PubSubClient.h>
#include <SimpleConnect.h>
#include <SimpleSleep.h>
#include <SimpleDebug.h>

// ##################################
// # Global declarations

bool DEBUG_ENABLED = true;
const int threshold = 700;
const char *ssid = "Apple Network 785";
const char *password = "";
const char *mqtt_server = "192.168.1.80";
const char *prefix = "ttap_sensor_";
const int settings_count = 2;
int settings_set = 0;

// TRACKING
String client_name;
unsigned long startTime;
unsigned long gracePeriodStart;
bool checking;
bool publishNow;
bool needs_water;

// SETTINGS
int check_duration = 30;
String check_time = "03:00";


PubSubClient client(espClient);

// ##################################
// # Functions

// Reset the state of the Application
// (should also be run at the start to normalize behavior)
void reset(){
  debug("State reseting");
  client_name = prefix + generateRandomID(6);
  info("Client name:", client_name);

  // normalize state
  startTime = 0;
  gracePeriodStart = 0;
  checking = false;
  publishNow = false;
  needs_water = false;
  settings_set = 0;
}

// Callback function to handle incoming messages
void callback(char* topic, byte* payload, unsigned int length) {
  String message;
  for (unsigned int i = 0; i < length; i++) {
    message += (char)payload[i];
  }
  String topicStr = String(topic);

  debug("Message arrived [", topicStr, "]: ", message);

  // Update settings from MQTT
  if (topicStr.startsWith("settings/home/sensor")) {
    if (topicStr.endsWith("/check_time")) {
      check_time = message;
      debug("Updated check time:", check_time);
      settings_set++;
    }
    if (topicStr.endsWith("/check_duration")) {
      check_duration = message.toInt();
      debug("Updated check duration:", check_duration);
      settings_set++;
    }  
  }
}

// Function to reconnect to the MQTT broker
void reconnect() {
  while (!client.connected()) {
    debug("Attempting MQTT connection...");
    if (client.connect(client_name.c_str())) {
      debug("MQTT connected");
      client.subscribe("settings/home/sensor/#");
    } else {
      debug("Failed to connect to MQTT rc=", client.state(), "try again in 1 second");
      delay(1000);
    }
  }
}

// ##################################
// # Main

void setup() {
  Serial.begin(9600);
  // Give time to boot
  while (!Serial) { }

  setup_wifi(ssid, password);
  setupTime();

  // Initialize LED as Output
  pinMode(LED_BUILTIN, OUTPUT);

  // Initialize MQTT
  client.setServer(mqtt_server, 1883);
  client.setCallback(callback);

  reset();
}

void loop() {
  long now = millis();
  
  if (checking) {
    // turn on the LED while we operate
    digitalWrite(LED_BUILTIN, LOW);
    // extract the sensor value from the A0 Channel
    // the value will normaly be between 300 and 700
    // while 700 is close to dry
    int sensorValue = analogRead(A0);
    // compare to threshold
    needs_water = (sensorValue > threshold);
    
    // Check for 0.5 minutes (30000 millisecounds)
    if (now - startTime >= 30000) {
      checking = false;
      publishNow = true;
      debug("Check completed, needs water:", needs_water);
    }
    return;
  }
  
  // ensure MQTT is always connected
  if (!client.connected()) {
    reconnect();
  }
  client.loop();

  // do not continue if settings are not set (updated)
  if (settings_set < settings_count) {
    return;
  }

  if (!publishNow && !checking){
    // update starttime to ensure the system can check for mositure over time
    // start when the settings have been recived 
    startTime = millis();
    checking = true;
    info("Starting check for moisture (30 seconds)");
    debug("Check duration: 30s, Started at:", startTime);
  }

  if (publishNow){
    if (gracePeriodStart == 0){
      // Publish the watering state
      String payload = needs_water ? "true" : "false";
      client.publish("home/sensor/watering_needed", payload.c_str());
      info("Published watering state:", payload);

      digitalWrite(LED_BUILTIN, HIGH);
      gracePeriodStart = now;
    }else if (now - gracePeriodStart > 1000){
      gracePeriodStart = 0;
      publishNow = false;
      
      // Sleep until next check time
      sleepUntil(check_time);
    }
  }
}
