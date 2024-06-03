use std::collections::HashMap;
use crate::{ topic, ClientModule };

pub struct ModuleManager {
    modules: Vec<Box<dyn ClientModule>>,
    configs: HashMap<String, String>,
}

impl ModuleManager {
    pub fn new() -> Self {
        tracing::trace!("ModuleManager created");

        Self {
            modules: Vec::new(),
            configs: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module: impl ClientModule + 'static) {
        let name = module.name();
        let settings = module
            .settings()
            .iter()
            .map(|(k, v)| { (format!("{}/{}", module.topic(), k), v.clone()) })
            .collect::<HashMap<String, String>>();

        self.configs.extend(settings);
        self.modules.push(Box::new(module));
        tracing::debug!("Registered module '{}'", name);
    }

    pub async fn handle_message(&self, topic: &str, payload: &str) {
        for ele in self.modules.iter().filter(|ele| topic.contains(&ele.topic())) {
            tracing::debug!("Handling message for module '{}' on topic '{}'", ele.name(), topic);
            ele.handle(topic, payload).await;
        }
    }

    pub fn initialize(&self, client: &rumqttc::AsyncClient) {
        for (topic, value) in &self.configs {
            let new_topic = format!("settings/{}", topic);
            let mut payload = value.to_string();
            if payload.is_empty() {
                continue;
            }
            // clean up the payload for strings
            payload = payload.replace("\"", "");

            // Publish the setting to the MQTT broker
            let _ = client.try_publish(
                new_topic,
                rumqttc::QoS::ExactlyOnce,
                true,
                payload.as_bytes()
            );
            tracing::debug!(
                "Published settings for '{}' retain = true, value = '{}'",
                topic,
                value
            );
        }

        for module in &self.modules {
            let _ = client.try_subscribe(topic!(module.topic(), "#"), rumqttc::QoS::ExactlyOnce);
            tracing::trace!("Subscribed to '{}'", module.topic());
        }
    }
}
