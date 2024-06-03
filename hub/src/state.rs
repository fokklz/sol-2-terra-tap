use serde::{ Deserialize, Serialize };

use crate::traits::ConfigFile;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub watering_needed: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            watering_needed: false,
        }
    }
}

impl ConfigFile<&'static str> for State {
    const PATH: &'static str = "state.json";
    type Config = Self;
}
