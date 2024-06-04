use std::fmt;

use chrono::{NaiveTime, Timelike};
use serde::{
    de::{self, Visitor},
    Deserializer, Serializer,
};

// Custom function to serialize NaiveTime to "HH:MM" format
pub fn serialize_naive_time<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format the NaiveTime as "HH:MM"
    let formatted = format!("{:02}:{:02}", time.hour(), time.minute());
    // Serialize the formatted string
    serializer.serialize_str(&formatted)
}

// Custom function to deserialize "HH:MM" format to NaiveTime
pub fn deserialize_naive_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    // Visitor to help with deserialization
    struct NaiveTimeVisitor;

    impl<'de> Visitor<'de> for NaiveTimeVisitor {
        type Value = NaiveTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string in the format HH:MM")
        }

        fn visit_str<E>(self, value: &str) -> Result<NaiveTime, E>
        where
            E: de::Error,
        {
            NaiveTime::parse_from_str(value, "%H:%M").map_err(de::Error::custom)
        }
    }

    // Use the visitor to deserialize the string
    deserializer.deserialize_str(NaiveTimeVisitor)
}

// Tests for the custom serialization and deserialization functions
#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    // Struct to test serialization and deserialization
    // Same as the actual usage in something like settings should be
    #[derive(Serialize, Deserialize)]
    struct Testing {
        #[serde(
            serialize_with = "serialize_naive_time",
            deserialize_with = "deserialize_naive_time"
        )]
        time: NaiveTime,
    }

    #[test]
    fn test_serialize_naive_time() {
        let time = NaiveTime::from_hms_opt(12, 34, 56).unwrap();
        let testing = Testing { time };
        let serialized = serde_json::to_string(&testing).unwrap();
        assert_eq!(serialized, r#"{"time":"12:34"}"#);
    }

    #[test]
    fn test_deserialize_naive_time() {
        let testing = serde_json::from_str::<Testing>(r#"{"time":"12:34"}"#).unwrap();
        assert_eq!(testing.time, NaiveTime::from_hms_opt(12, 34, 0).unwrap());
    }
}
