use serde::{Deserialize, Serialize};

use super::temperature_unit::TemperatureUnit;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Temperature {
    pub amount: f64,
    pub unit: TemperatureUnit,
}
