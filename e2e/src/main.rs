use std::{ io, process::{ Child, Command, Stdio }, sync::Arc, time::Duration };

/// Two settings for each client module
/// this is not a pattern but a coincidence
/// Tests will only start after all settings are received
const SETTINGS_COUNT: i64 = 4;

/// Trace the changes happening due to the messages of the tests
#[derive(Debug)]
struct Tracking {
    settings_received: i64,
    responses_received: i64,
    watering_needed_responses: i64,
}

use rumqttc::{ AsyncClient, MqttOptions };
use tokio::sync::{ broadcast, Mutex };

/// Spawn the Hub
fn spawn_hub() -> io::Result<Child> {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("hub")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(1);

    let tracking = Tracking {
        settings_received: 0,
        responses_received: 0,
        watering_needed_responses: 0,
    };
    let tracking = Arc::new(Mutex::new(tracking));
    let cloned_tracking = Arc::clone(&tracking);

    tracing::info!("Starting integration test using Simulated versions of the Clients");
    tracing::info!("Starting the Hub...");
    let mut hub = spawn_hub().expect("Failed to start hub");
    tracing::info!("Hub started successfully");

    tracing::info!("Setting up the simulated clients...");
    tracing::info!("Configuring a MQTT Client to simulate clients connecting to the Hub");
    let mut mqtt_options = MqttOptions::new("e2e-testing", "127.0.0.1", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    tracing::info!("Creating the MQTT Client...");
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    let client_clone = client.clone();
    tracing::info!("MQTT Client created successfully");
    tracing::info!("Spawning handler for the MQTT Client...");

    let eventloop_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                result = eventloop.poll() => {
                    if let Ok(notification) = result {
                        match notification {
                            rumqttc::Event::Incoming(incoming) => {
                                match incoming {
                                    rumqttc::Incoming::Publish(publish) => {
                                        let topic = publish.topic;
                                        let payload = publish.payload;
                                        
                                        let mut tlock = cloned_tracking.lock().await;
                                        if topic.starts_with("settings/") {
                                            let name = topic.trim_start_matches("settings/");
                                            tracing::info!("Received setting on topic '{}': {}", name, std::str::from_utf8(&payload).unwrap());
                                            tlock.settings_received += 1;
                                        } else if topic.ends_with("/response") {
                                            let name_without_response = topic.trim_end_matches("/response");
                                            let parsed = std::str::from_utf8(&payload).unwrap();
                                            tracing::info!("Received response on topic '{}': {}", name_without_response, parsed);
                                            tlock.responses_received += 1;
                                            if parsed == "true" {
                                                tlock.watering_needed_responses += 1;
                                            }
                                        }
                                        drop(tlock);
                                    }
                                    rumqttc::Incoming::ConnAck(_) => {
                                        tracing::info!("Connected to MQTT broker");
                                        client_clone.subscribe("#", rumqttc::QoS::ExactlyOnce).await.unwrap();
                                        tracing::info!("Subscribed to all topics");
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutting down MQTT client...");
                    break;
                }
            }
        }
    });

    tracing::info!("Task spawned successfully");
    tracing::info!("Waiting for the MQTT Client to connect to the Hub...");
    tracing::info!("Starting the tests...");
    tracing::info!("Waiting for all settings to be received...");
    let mut settings_received = 0;
    while settings_received < SETTINGS_COUNT {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let tlock = tracking.lock().await;
        settings_received = tlock.settings_received;
    }
    tracing::info!("All settings received");

    // ################
    // Watering Tests
    tracing::info!("---------------- Watering Tests ----------------");
    let mut watering_test_count = 0;
    let mut watering_test_passed = 0;

    let tlock = tracking.lock().await;
    let current_responses = tlock.responses_received.clone();
    drop(tlock);

    watering_test_count += 1;
    tracing::info!("Sending a message to the HUB to request the state");
    client
        .publish(
            "home/watering/watering_needed",
            rumqttc::QoS::AtMostOnce,
            false,
            "".as_bytes()
        ).await
        .unwrap();

    let mut responses_received = 0;
    tracing::info!("Waiting for the response...");
    while current_responses + 1 != responses_received {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let tlock = tracking.lock().await;
        responses_received = tlock.responses_received;
    }
    // because the state response has been received
    watering_test_passed += 1;

    tracing::info!("Watering tests completed");
    tracing::info!("Watering tests passed: {}/{}", watering_test_passed, watering_test_count);

    // ################
    // Sensor Tests
    tracing::info!("---------------- Sensor Tests ----------------");
    let mut sensor_test_count = 0;
    let mut sensor_test_passed = 0;
    let tlock = tracking.lock().await;
    let current_watering_responses = tlock.watering_needed_responses.clone();
    let current_responses = tlock.responses_received.clone();
    drop(tlock);
    sensor_test_count += 1;
    tracing::info!("Sending a message to the hub to confirm watering is needed");
    client
        .publish(
            "home/sensor/watering_needed",
            rumqttc::QoS::AtMostOnce,
            false,
            "true".as_bytes()
        ).await
        .unwrap();
    sensor_test_count += 1;
    // TODO: there may be improvments to be made for this test, as we maybe could directly extract the current state for the hub rather than relying on a other module
    tracing::info!("Test if message was sent successfully, by requesting the watering state");
    client
        .publish(
            "home/watering/watering_needed",
            rumqttc::QoS::AtMostOnce,
            false,
            "".as_bytes()
        ).await
        .unwrap();

    let mut responses_received = 0;
    tracing::info!("Waiting for the response...");
    while current_responses + 1 != responses_received {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let tlock = tracking.lock().await;
        responses_received = tlock.responses_received;
    }
    // because the watering response has been received
    sensor_test_passed += 1;
    let tlock = tracking.lock().await;
    let watering_needed_responses = tlock.watering_needed_responses;
    if watering_needed_responses == current_watering_responses + 1 {
        // because the state was changed (is true on the HUB)
        sensor_test_passed += 1;
    }
    drop(tlock);
    tracing::info!("Sensor tests completed");
    tracing::info!("Sensor tests passed: {}/{}", sensor_test_passed, sensor_test_count);

    // ################
    // Cleanup
    tracing::info!("---------------- Cleanup ----------------");
    tracing::info!("Shutting down the MQTT client...");
    shutdown_tx.send(()).unwrap();
    eventloop_handle.await.unwrap();
    tracing::info!("MQTT client shut down successfully");
    tracing::info!("Killing the Hub...");
    hub.kill().unwrap();
    tracing::info!("Hub killed successfully");
    tracing::info!("E2E test completed");
    tracing::info!(
        "Tests passed: {}/{}",
        watering_test_passed + sensor_test_passed,
        watering_test_count + sensor_test_count
    );
}
