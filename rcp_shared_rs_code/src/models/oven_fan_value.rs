use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OvenFanValue {
    Off,
    Low,
    High,
}

impl OvenFanValue {
    pub fn from_database_code(i: i32) -> Option<OvenFanValue> {
        match i {
            0 => Some(OvenFanValue::Off),
            1 => Some(OvenFanValue::Low),
            2 => Some(OvenFanValue::High),
            _ => None,
        }
    }

    pub fn to_database_code(&self) -> i32 {
        match self {
            OvenFanValue::Off => 0,
            OvenFanValue::Low => 1,
            OvenFanValue::High => 2,
        }
    }

    pub fn from_string(s: &str) -> Option<OvenFanValue> {
        match s.to_lowercase().as_str() {
            "off" => Some(OvenFanValue::Off),
            "low" => Some(OvenFanValue::Low),
            "high" => Some(OvenFanValue::High),
            _ => None,
        }
    }

    pub fn to_string(v: &Option<OvenFanValue>) -> String {
        match v {
            Some(OvenFanValue::Off) => "Off".to_string(),
            Some(OvenFanValue::Low) => "Low".to_string(),
            Some(OvenFanValue::High) => "High".to_string(),
            _ => "".to_string(),
        }
    }
}
