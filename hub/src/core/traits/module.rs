use std::collections::HashMap;

/// A trait for modules that can be added to the Hub
#[async_trait::async_trait]
pub trait ClientModule: Send + Sync {
    /// The name of the module (last part of the topic by default)
    fn name(&self) -> String {
        self.topic()
            .clone()
            .split('/')
            .last()
            .unwrap_or_default()
            .to_string()
    }

    /// The topic the module is interested in
    fn topic(&self) -> String;

    /// The handlers for the module
    async fn handle(&self, topic: &str, payload: &str);

    /// The settings for the module
    fn settings(&self) -> HashMap<String, String>;
}
