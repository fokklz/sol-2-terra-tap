# Clients (ESP8266 Chips)

This directory contains the code for the ESP8266 chips that are used in terra-tap.

## Components

The following components are included in this directory:

| Component  | Description                                                                                                                                                                       |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sensor`   | The sensor component is responsible for interpreting the sensor data and sending the state to the server.                                                                         |
| `watering` | The watering component is responsible for opening and closing the water valve. It requests the state from the server and opens the valve if the state is `watering_needed: true`. |

## Installation / Flashing

You need to configure your Arduino IDE to work with the ESP8266 chips.
This can be achieved by installing the folling Board Manager URL: `http://arduino.esp8266.com/stable/package_esp8266com_index.json`.

After you installed the [lib](./lib/), you can open the files in the Arduino IDE and flash them to the ESP8266 chips.

Ensure that the COM port is set correctly in the Arduino IDE.