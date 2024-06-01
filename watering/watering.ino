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
const char *prefix = "ttap_watering_";
const int settings_count = 2;
int settings_set = 0;

// TRACKING
String client_name;
unsigned long startTime;
bool needs_water;
bool requested;

// SETTINGS
int open_duration = 5 * 60;
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
  needs_water = false;
  requested = false;
  settings_set = 0;
  digitalWrite(LED_BUILTIN, HIGH);
  digitalWrite(D1, LOW);
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
  if (topicStr.startsWith("settings/home/watering")) {
    if (topicStr.endsWith("/check_time")) {
      check_time = message;
      debug("Updated check time:", check_time);
      settings_set++;
    }
    if (topicStr.endsWith("/open_duration")) {
      open_duration = message.toInt();
      debug("Updated open duration:", open_duration);
      settings_set++;
    }  
  }else if (topicStr.startsWith("home/watering")){
    if (topicStr.endsWith("/response")){
      message.toLowerCase();
      if (message == "true"){
        needs_water = true;
      }else{
        needs_water = false;
      }
      
      startTime = millis();
    }
  }
}

// Function to reconnect to the MQTT broker
void reconnect() {
  while (!client.connected()) {
    debug("Attempting MQTT connection...");
    if (client.connect(client_name.c_str())) {
      debug("MQTT connected");
      client.subscribe("settings/home/watering/#");
      client.subscribe("home/watering/#/response");
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
  // Pin connected to relais
  pinMode(D1, OUTPUT);

  // Initialize MQTT
  client.setServer(mqtt_server, 1883);
  client.setCallback(callback);

  reset();
}

void loop() {
  long now = millis();
  if (startTime > 0) {
    if (needs_water){
      digitalWrite(LED_BUILTIN, LOW);
      digitalWrite(D1, HIGH);

      if (now - startTime > open_duration * 1000){
          reset();
          sleepUntil(check_time);
      }
    }else{
      reset();
      sleepUntil(check_time);
    }
    return;
  }

  // ensure MQTT is always connected
  if (!client.connected()) {
    reconnect();
  }
  client.loop();

  if (settings_set < settings_count){
    // wait for settings to be set
    return;
  }

  if (!requested){
    debug("Checking for water needs...");
    // request watering state
    client.publish("home/watering/watering_needed", "");
    requested = true;
  }
}
