#ifndef SIMPLE_CONNECT_H
#define SIMPLE_CONNECT_H

#include <ESP8266WiFi.h>
#include <time.h>
#include <Arduino.h>
#include <SimpleDebug.h>

// WiFi and MQTT client instances
extern WiFiClient espClient;

// Function declarations
void setupTime();
void setup_wifi(const char* ssid, const char* password);
String generateRandomID(int length);

#endif // SIMPLE_CONNECT_H
