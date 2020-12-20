use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    pub fn from_string(s: &str) -> Option<TemperatureUnit> {
        match s.to_lowercase().as_str() {
            "celsius" => Some(TemperatureUnit::Celsius),
            "fahrenheit" => Some(TemperatureUnit::Fahrenheit),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TemperatureUnit::Celsius => "Celsius".to_string(),
            TemperatureUnit::Fahrenheit => "Fahrenheit".to_string(),
        }
    }
}
