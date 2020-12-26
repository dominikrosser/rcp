use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum OvenFanValue {
    Off,
    Low,
    High,
}

impl Default for OvenFanValue {
    fn default() -> Self {
        OvenFanValue::Off
    }
}

impl FromStr for OvenFanValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(OvenFanValue::Off),
            "low" => Ok(OvenFanValue::Low),
            "high" => Ok(OvenFanValue::High),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OvenFanValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OvenFanValue::Off => fmt.write_str("Off")?,
            OvenFanValue::Low => fmt.write_str("Low")?,
            OvenFanValue::High => fmt.write_str("High")?,
        };
        Ok(())
    }
}
