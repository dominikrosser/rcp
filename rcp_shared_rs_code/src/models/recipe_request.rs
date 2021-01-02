use serde::{Deserialize, Serialize};
use std::default::Default;

use super::book_source::BookSource;
use super::ingredient::Ingredient;
use super::oven_fan_value::OvenFanValue;
use super::r#yield::Yield;
use super::step::Step;
use super::temperature::Temperature;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RecipeRequest {
    pub recipe_name: Option<String>,
    pub oven_time: Option<f64>,
    pub notes: Option<String>,
    pub oven_fan: Option<OvenFanValue>,
    pub oven_temp: Option<Temperature>,
    pub source_book: Option<BookSource>,
    pub source_authors: Option<Vec<String>>,
    pub source_url: Option<String>,
    pub ingredients: Option<Vec<Ingredient>>,
    pub steps: Option<Vec<Step>>,
    pub yields: Option<Vec<Yield>>,
}

impl RecipeRequest {
    pub fn new() -> Self {
        Self {
            yields: Some(vec![Yield::new()]),
            ..Default::default()
        }
    }
}
