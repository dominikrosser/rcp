use serde::{Deserialize, Serialize};

use super::oven_fan_value::OvenFanValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeRequest {
    pub recipe_name: String,
    pub oven_time: Option<f64>,
    pub notes: Option<String>,
    pub oven_fan: Option<OvenFanValue>,
}
