use std::time::Duration;

use once_cell::sync::OnceCell;
use rumqttc::{AsyncClient, MqttOptions};
use tokio::sync::{broadcast::Receiver, Mutex};
use tracing::span;

pub static CLIENT: OnceCell<Mutex<AsyncClient>> = OnceCell::new();

pub async fn run(mut shutdown: Receiver<()>) -> tokio::task::JoinHandle<()> {
    let broker_span = span!(tracing::Level::INFO, "mqtt-client");
    let _ = broker_span.enter();

    let mut mqtt_options = MqttOptions::new("hub", "127.0.0.1", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    CLIENT.set(Mutex::new(client.clone())).unwrap();
    tracing::info!("Client created and stored in global static variable");

    tokio::spawn(async move {
        loop {
            tokio::select! {
                result = eventloop.poll() => {
                    if let Ok(notification) = result {
                        //dbg!("Received = {:#?}", notification.clone());
                        match notification {
                            rumqttc::Event::Incoming(incoming) => {
                                match incoming {
                                    rumqttc::Incoming::Publish(publish) => {
                                        let topic = publish.topic;
                                        let payload = publish.payload;
                                        tracing::info!("Topic = {:#?}, Received = {:#?}", topic, payload);
                                    }
                                    rumqttc::Incoming::ConnAck(_) => {
                                        // Initialize the modules once the connection is acknowledged
                                    }
                                    // for now any other messages are just irgnored
                                    _ => {
                                        //tracing::info!("Received = {:#?}", incoming);
                                    }
                                }
                            }
                            rumqttc::Event::Outgoing(_outgoing) => {
                                //tracing::info!("Sent = {:#?}", outgoing);
                            }
                        }
                    }
                }
                _ = shutdown.recv() => {
                    tracing::info!("Shutting down...");
                    break;
                }
            }
        }
    })
}
