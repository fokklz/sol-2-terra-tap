use crate::{ topic, ClientModule };
use std::collections::HashMap;

/// Core manager for handling modules and the settings for modules
/// All modules should be registered with the manager
/// The handle_message function should be called from the main loop
pub struct ModuleManager {
    modules: Vec<Box<dyn ClientModule>>,
    configs: HashMap<String, String>,
}

impl ModuleManager {
    /// Create a new ModuleManager
    pub fn new() -> Self {
        tracing::trace!("ModuleManager created");

        Self {
            modules: Vec::new(),
            configs: HashMap::new(),
        }
    }

    /// Register a module with the manager
    /// This will add the settings to the settings map
    pub fn register_module(&mut self, module: impl ClientModule + 'static) {
        let name = module.name();
        // prefix the setting with the module topic
        let settings = module
            .settings()
            .iter()
            .map(|(k, v)| (format!("{}/{}", module.topic(), k), v.clone()))
            .collect::<HashMap<String, String>>();

        self.configs.extend(settings);
        self.modules.push(Box::new(module));
        tracing::debug!("Registered module '{}'", name);
    }

    /// Handle a message from the MQTT broker
    /// Will filter the modules based on the topic and call the handle function of each matching module
    pub async fn handle_message(&self, topic: &str, payload: &str) {
        for ele in self.modules.iter().filter(|ele| topic.contains(&ele.topic())) {
            tracing::debug!("Handling message for module '{}' on topic '{}'", ele.name(), topic);
            ele.handle(topic, payload).await;
        }
    }

    /// Initialize the modules and settings
    /// Subscribes to the topics of the modules and publishes the settings as retained messages
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
            let res = client.try_publish(
                new_topic,
                rumqttc::QoS::ExactlyOnce,
                true,
                payload.as_bytes()
            );

            if res.is_ok() {
                tracing::debug!(
                    "Published settings for '{}' retain = true, value = '{}'",
                    topic,
                    value
                );
            } else {
                tracing::error!(
                    "Failed to publish settings for '{}' retain = true, value = '{}'",
                    topic,
                    value
                );
            }
        }

        for module in &self.modules {
            let res = client.try_subscribe(topic!(module.topic(), "#"), rumqttc::QoS::ExactlyOnce);

            if res.is_ok() {
                tracing::debug!("Subscribed to '{}'", module.topic());
            } else {
                tracing::error!("Failed to subscribe to '{}'", module.topic());
            }
        }
    }
}

/// Tests for the ModuleManager
/// The Initialize function is not tested as it requires a rumqttc::AsyncClient
/// The handle_message function is not tested as it returns nothing
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    struct TestModule {
        config_data: String,
    }

    #[async_trait::async_trait]
    impl ClientModule for TestModule {
        fn topic(&self) -> String {
            "test/topic".to_string()
        }

        async fn handle(&self, _topic: &str, _payload: &str) {}

        fn settings(&self) -> HashMap<String, String> {
            let mut settings = HashMap::new();

            let ser = serde_json::to_string(&self).unwrap();
            let deser = serde_json
                ::to_value(serde_json::from_str::<TestModule>(&ser).unwrap())
                .unwrap();

            settings.insert("config_data".to_string(), deser["config_data"].to_string());

            settings
        }
    }

    #[test]
    fn test_module_manager_new() {
        let manager = ModuleManager::new();
        assert_eq!(manager.modules.len(), 0);
        assert_eq!(manager.configs.len(), 0);
    }

    #[test]
    fn test_register_module() {
        let mut manager = ModuleManager::new();
        let module = TestModule::default();
        manager.register_module(module);

        assert_eq!(manager.modules.len(), 1);
        assert_eq!(manager.configs.len(), 1);
    }

    #[test]
    fn test_setting_prefix_correctly(){
        let mut manager = ModuleManager::new();
        let module = TestModule::default();
        manager.register_module(module);

        assert!(manager.configs.contains_key("test/topic/config_data"));
    }
}
