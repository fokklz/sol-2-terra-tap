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
