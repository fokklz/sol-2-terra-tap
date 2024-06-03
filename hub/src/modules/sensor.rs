use std::collections::HashMap;

use chrono::{ Duration, NaiveTime };

use super::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SensorModule {
    #[serde(
        serialize_with = "crate::serde::serialize_naive_time",
        deserialize_with = "crate::serde::deserialize_naive_time"
    )]
    pub check_time: NaiveTime,
    pub check_duration: u64,
}

impl Default for SensorModule {
    fn default() -> Self {
        Self::from(&Settings::default())
    }
}

impl From<&Settings> for SensorModule {
    fn from(settings: &Settings) -> Self {
        Self {
            // remove 5 minutes from the check time for the sensor
            check_time: settings.check_time - Duration::minutes(5),
            check_duration: settings.check_duration,
        }
    }
}

#[async_trait::async_trait]
impl ClientModule for SensorModule {
    fn topic(&self) -> String {
        format!("{}/{}", super::PREFIX, "sensor")
    }

    async fn handle(&self, topic: &str, payload: &str) {
        match topic {
            t if t == topic!(self.topic(), "watering_needed") => {
                if payload.to_ascii_lowercase() == "true" {
                    let mut state_mut = crate::STATE.get().unwrap().lock().await;
                    state_mut.watering_needed = true;
                    tracing::info!("Watering needed: {}", state_mut.watering_needed);
                } else {
                    tracing::trace!(
                        "Received message on topic '{}' with payload '{}'",
                        topic,
                        payload
                    );
                }
            }
            _ => {}
        }
    }

    fn settings(&self) -> HashMap<String, String> {
        let mut settings = HashMap::new();

        let ser = serde_json::to_string(&self).unwrap();
        let deser = serde_json
            ::to_value(serde_json::from_str::<SensorModule>(&ser).unwrap())
            .unwrap();

        for (key, value) in deser.as_object().unwrap() {
            settings.insert(key.clone(), value.to_string());
        }

        settings
    }
}
