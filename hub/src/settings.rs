use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::traits::ConfigFile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(
        serialize_with = "crate::serde::serialize_naive_time",
        deserialize_with = "crate::serde::deserialize_naive_time"
    )]
    pub check_time: NaiveTime,
    pub check_duration: u64,
    pub open_duration: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // Default time is 3:00 AM
            check_time: NaiveTime::from_hms_opt(3, 0, 0).unwrap(),
            // Default duration is 30 seconds
            check_duration: 30,
            // Default duration is 5 minutes
            open_duration: 5 * 60,
        }
    }
}

impl ConfigFile<&'static str> for Settings {
    const PATH: &'static str = "settings.json";
    type Config = Self;
}
