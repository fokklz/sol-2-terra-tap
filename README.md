# Terra Tap

**ALPHA VERSION. PRODUCTION USE NOT RECOMMENDED.**

## Overview
TerraTap is an innovative project aimed at optimizing water usage in urban ecosystems. Traditional irrigation systems often operate on predefined schedules that do not consider the actual soil moisture or the specific water needs of plants, leading to both overwatering and underwatering. TerraTap addresses these issues by developing a smart irrigation system that leverages soil moisture sensors and adaptive control technology to ensure precise and demand-based water supply.

## Project Structure

The project is divided into the following components:

- **HUB**: The central point. It keeps track of the state and allows states to be requested. It also start a MQTT broker responsible for the communication between the clients.
- **MQTTD**: The MQTT broker. Responsible for the communication between the clients. Started by the HUB.
- **E2E**: End-to-end tests. They start the HUB and the Broker and imitate a client connecting and communicating with the HUB (over the broker).
- **Clients**: The clients are the devices that connect to the HUB. They can be sensors, actuators, or other devices that need to communicate with the HUB. The lib holds the common code for the clients. The clients are implemented in separate folders. 

## Scope
The project is focused on small to medium-sized urban and suburban green spaces, excluding large agricultural fields and indoor applications like greenhouses.

## How to Contribute
Contributors can help by providing feedback on system design, testing methodologies, and code optimizations. For more information on contributing, please refer to the `CONTRIBUTING.md` file.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
