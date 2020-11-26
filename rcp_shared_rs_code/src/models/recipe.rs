use serde::{Deserialize, Serialize};

struct Test();

#[derive(Serialize, Deserialize, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    amount: f32,
    unit: TemperatureUnit,
}
