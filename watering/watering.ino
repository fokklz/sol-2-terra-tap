#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include <time.h>
#include <TimeLib.h> 

const int threshold = 700;
const char *ssid = "Apple Network 785";
const char *password = "";
const char *mqtt_server = "192.168.1.80";

const char *prefix = "ttap_watering_";
size_t random_length = 8;
size_t total_length = strlen(prefix) + random_length;
char *client_name = (char *)malloc(total_length + 1);

// Create WiFi and MQTT clients
WiFiClient espClient;
PubSubClient client(espClient);

// Configure the correct time
void setupTime() {
  // setoff for 2 hours
  configTime(2*3600, 0, "pool.ntp.org", "time.nist.gov");
  // Wait until time is set
  while (time(nullptr) < 8 * 3600 * 2) { 
      delay(100);
  }
}

// Function to connect to Wi-Fi
void setup_wifi() {
  Serial.println();
  Serial.print("Connecting to ");
  Serial.println(ssid);

  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }

  Serial.println("");
  Serial.println("WiFi connected");
  Serial.print("IP address: ");
  Serial.println(WiFi.localIP());
}

// Function to generate a random alphanumeric string
void generate_random_string_with_prefix(char *output_string, const char *prefix, size_t random_length) {
    const char charset[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    size_t charset_size = sizeof(charset) - 1;
    size_t prefix_length = strlen(prefix);

    // Copy prefix to output_string
    strcpy(output_string, prefix);

    // Generate random part of the string
    for (size_t i = 0; i < random_length; i++) {
        int random_index = os_random() % charset_size;
        output_string[prefix_length + i] = charset[random_index];
    }

    // Null-terminate the string
    output_string[prefix_length + random_length] = '\0';
}

// Callback function to handle incoming messages
void callback(char* topic, byte* payload, unsigned int length) {
  String message;
  for (unsigned int i = 0; i < length; i++) {
    message += (char)payload[i];
  }

  String topicStr = String(topic);
}

// Function to reconnect to the MQTT broker
void reconnect() {
  while (!client.connected()) {
    Serial.print("Attempting MQTT connection...");
    if (client.connect(client_name)) {
      Serial.println("connected");
    } else {
      Serial.print("failed, rc=");
      Serial.print(client.state());
      Serial.println(" try again in 1 second");
      delay(1000);
    }
  }
}

void setup() {
  Serial.begin(9600);
  // allow the system to boot
  delay(10);
  generate_random_string_with_prefix(client_name, prefix, random_length);
  setup_wifi();
  setupTime();

  // Initialize LED as Output
  pinMode(LED_BUILTIN, OUTPUT);
  
  client.setServer(mqtt_server, 1883);
  client.setCallback(callback);
}

void loop() {
  // ensure MQTT is always connected
  if (!client.connected()) {
    reconnect();
  }
  client.loop();
}
