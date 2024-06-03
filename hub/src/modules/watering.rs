use std::collections::HashMap;

use super::prelude::*;

use chrono::NaiveTime;
use rumqttc::QoS;

#[derive(Serialize, Deserialize)]
pub struct WateringModule {
    #[serde(
        serialize_with = "crate::serde::serialize_naive_time",
        deserialize_with = "crate::serde::deserialize_naive_time"
    )]
    pub check_time: NaiveTime,
    pub open_duration: u64,
}

impl Default for WateringModule {
    fn default() -> Self {
        Self::from(&Settings::default())
    }
}

impl From<&Settings> for WateringModule {
    fn from(settings: &Settings) -> Self {
        Self {
            check_time: settings.check_time,
            open_duration: settings.open_duration,
        }
    }
}

#[async_trait::async_trait]
impl ClientModule for WateringModule {
    fn topic(&self) -> String {
        format!("{}/{}", super::PREFIX, "watering")
    }

    fn settings(&self) -> HashMap<String, String> {
        let mut settings = HashMap::new();

        let ser = serde_json::to_string(&self).unwrap();
        let deser = serde_json
            ::to_value(serde_json::from_str::<WateringModule>(&ser).unwrap())
            .unwrap();

        for (key, value) in deser.as_object().unwrap() {
            settings.insert(key.clone(), value.to_string());
        }

        settings
    }

    async fn handle(&self, topic: &str, _payload: &str) {
        match topic {
            t if t == topic!(self.topic(), "watering_needed") => {
                let client = crate::CLIENT.get().unwrap().lock().await;
                let mut state = crate::STATE.get().unwrap().lock().await;
                // Publish the watering needed state to the MQTT broker so the client can read it
                // TODO: Tie this to a client to allow for multiple watering modules
                let res = client.try_publish(
                    topic!(self.topic(), "watering_needed/response"),
                    QoS::ExactlyOnce,
                    false,
                    state.watering_needed.clone().to_string().as_bytes()
                );

                // If the publish was successful, reset the watering needed state
                if res.is_ok() {
                    state.watering_needed = false;
                    tracing::trace!("Watering needed reset");
                }
            }
            _ => {}
        }
    }
}
