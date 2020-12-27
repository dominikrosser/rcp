use serde::{Deserialize, Serialize};
pub use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        TemperatureUnit::Celsius
    }
}

impl FromStr for TemperatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" => Ok(TemperatureUnit::Celsius),
            "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemperatureUnit::Celsius => fmt.write_str("Celsius")?,
            TemperatureUnit::Fahrenheit => fmt.write_str("Fahrenheit")?,
        };
        Ok(())
    }
}
