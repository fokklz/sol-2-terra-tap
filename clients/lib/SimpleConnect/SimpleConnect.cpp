#include "SimpleConnect.h"

// WiFi and MQTT client instances
WiFiClient espClient;

// Configure the correct time
void setupTime() {
  debug("Syncing time...");
  // setoff for 2 hours
  configTime(2*3600, 0, "pool.ntp.org", "time.nist.gov");
  // Wait until time is set
  while (time(nullptr) < 8 * 3600 * 2) { 
      delay(100);
  }

  time_t now = time(nullptr);
  struct tm* timeinfo;
  timeinfo = localtime(&now);
  char timeStr[9];
  strftime(timeStr, sizeof(timeStr), "%H:%M:%S", timeinfo);
  info("System time:", timeStr); 
}

// Function to connect to Wi-Fi
void setup_wifi(const char* ssid, const char* password) {
  info("Connecting to WiFi:", ssid);
  
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    // fallback to Serial.print to not clutter the debug output
    Serial.print(".");
  }
  Serial.println();

  info("WiFi connected, IP address:", WiFi.localIP());
}

// Function to generate a random alphanumeric string
String generateRandomID(int length) {
  const char charset[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  String randomString = "";
  // Initialize random number generator seed from an unconnected pin
  randomSeed(analogRead(0));
  
  for (int i = 0; i < length; i++) {
    int index = random(0, sizeof(charset) - 1);
    randomString += charset[index];  
  }

  return randomString;
}