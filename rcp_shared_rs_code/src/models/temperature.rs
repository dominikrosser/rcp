use serde::{Deserialize, Serialize};

use super::temperature_unit::TemperatureUnit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    amount: f32,
    unit: TemperatureUnit,
}
