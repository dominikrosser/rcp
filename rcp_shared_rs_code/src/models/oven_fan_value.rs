use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OvenFanValue {
    Off,
    Low,
    High,
}

impl OvenFanValue {
    pub fn from_string(s: &str) -> Option<OvenFanValue> {
        match s.to_lowercase().as_str() {
            "off" => Some(OvenFanValue::Off),
            "low" => Some(OvenFanValue::Low),
            "high" => Some(OvenFanValue::High),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OvenFanValue::Off => "Off".to_string(),
            OvenFanValue::Low => "Low".to_string(),
            OvenFanValue::High => "High".to_string(),
        }
    }
}
