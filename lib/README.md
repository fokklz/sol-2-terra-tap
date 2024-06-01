# Ardunio Libraries

This folder contains the libraries that are used by the components of the project. The libraries facilitate common functionalities that are used by multiple components. 

## Libraries

The following libraries are included in this folder:
| Library         | Description                                                                                                                                                                                  |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SimpleDebug`   | A simple debugging library that allows for the printing of debug messages to the serial monitor. It provides a Syntax similar to `console.log` of javascript.                                |
| `SimpleConnect` | A simple library that allows for the connection to a WiFi network. It also provides a additional helper function to simplify the process of generating a client ID.                          |
| `SimpleSleep`   | A simple library that allows for the ESP32 to enter deep sleep mode. The function `sleepUntil` expects a time string eg. `HH:MM` and will put the ESP32 to sleep until then. on the next day |

## Installation

To install the libraries, run the `install.ps1` script in the root of this directory. This script will copy the libraries to the correct location in the Arduino IDE library folder. Ensure that the Arduino IDE is closed before running the script (or restart the IDE after running the script).