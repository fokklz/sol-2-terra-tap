use std::path::Path;

/// A trait for loading and saving configuration files
pub trait ConfigFile<P: AsRef<Path>> {
    type Config: serde::de::DeserializeOwned + Default + serde::Serialize;

    const PATH: P;

    /// Load settings from the file or return default settings
    fn load() -> Self::Config {
        std::fs::read_to_string(Self::PATH.as_ref())
            .map(|s| serde_json::from_str::<Self::Config>(&s).unwrap())
            .unwrap_or_default()
    }

    /// Save settings to the file
    fn save(&self)
    where
        Self: serde::Serialize,
    {
        let s = serde_json::to_string(self).unwrap();
        std::fs::write(Self::PATH.as_ref(), s).unwrap();
    }
}

/// Tests for the ConfigFile trait above
/// Creates a TestConfig struct that implements the ConfigFile trait
/// and tests the load and save methods
/// The test file is created in the current directory and removed after the test
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    struct TestConfig {
        pub test: String,
    }

    impl ConfigFile<&'static str> for TestConfig {
        type Config = TestConfig;

        const PATH: &'static str = "test.json";
    }

    #[test]
    fn test_config_file_load() {
        let config = TestConfig::load();
        assert_eq!(config.test, "");
    }

    /// This will also test the load from a file
    #[test]
    fn test_config_file_save() {
        let mut config = TestConfig::default();
        assert_eq!(config.test, "");

        config.test = "test".to_string();
        config.save();

        let config_loaded = TestConfig::load();
        assert_eq!(config_loaded.test, "test");

        // Clean up test file
        std::fs::remove_file("test.json").unwrap();
    }
}
