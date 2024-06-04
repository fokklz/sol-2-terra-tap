use std::collections::HashMap;

/// A trait for modules that can be added to the Hub
#[async_trait::async_trait]
pub trait ClientModule: Send + Sync {
    /// The name of the module (last part of the topic by default)
    fn name(&self) -> String {
        self.topic().clone().split('/').last().unwrap_or_default().to_string()
    }

    /// The topic the module is interested in
    fn topic(&self) -> String;

    /// The handlers for the module
    async fn handle(&self, topic: &str, payload: &str);

    /// The settings for the module
    fn settings(&self) -> HashMap<String, String>;
}

/// Test the ClientModule trait with a simple module
#[cfg(test)]
mod simple_module_tests {
    use super::*;

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    struct SimpleTestModule;

    #[async_trait::async_trait]
    impl ClientModule for SimpleTestModule {
        fn topic(&self) -> String {
            "test/topic".to_string()
        }

        async fn handle(&self, _topic: &str, _payload: &str) {}

        fn settings(&self) -> HashMap<String, String> {
            HashMap::new()
        }
    }

    #[test]
    fn test_simple_client_module() {
        let module = SimpleTestModule;
        assert_eq!(module.name(), "topic");
        assert_eq!(module.topic(), "test/topic");
        assert_eq!(module.settings().len(), 0);
    }
}

/// Test the ClientModule trait with a module that has settings
#[cfg(test)]
mod module_tests {
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

            for (key, value) in deser.as_object().unwrap() {
                settings.insert(key.clone(), value.to_string());
            }
            settings
        }
    }

    #[test]
    fn test_client_module() {
        let module = TestModule::default();
        assert_eq!(module.name(), "topic");
        assert_eq!(module.topic(), "test/topic");
        assert_eq!(module.settings().len(), 1);
    }
}
