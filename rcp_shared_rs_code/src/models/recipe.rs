use serde::{Deserialize, Serialize};

use super::book_source::BookSource;
use super::ingredient::Ingredient;
use super::oven_fan_value::OvenFanValue;
use super::r#yield::Yield;
use super::step::Step;
use super::temperature::Temperature;
use super::temperature_unit::TemperatureUnit;

// See Open Recipe Format
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    // recipe_uuid
    pub recipe_uuid: String,

    /* The name of this recipe. */
    pub recipe_name: Option<String>,

    /* Setting to be used with convection oven. Possible values are “Off”, “Low” and “High”. If not specified, it is assumed to be “Off”.
     * If specified, all software should display and print this value. If not specified, it is up to the software whether or not it is displayed and/or printed,
     * but it should be consistent. */
    pub oven_fan: Option<OvenFanValue>,

    /* Starting oven temperature, if the oven is used. */
    pub oven_temp: Option<Temperature>,

    /* How long the dish should spend in the oven.
     * This is an overall value, which refers to the recipe as a whole. If multiple oven times are used, they should be specified in the recipe. */
    pub oven_time: Option<f64>,

    /* A list of dicts, defining which food items are to be added to the recipe. These items should be listed in the order in which they are to be used.
     * Bearing this in mind, a particular item may be listed multiple times, if it is to be used multiple times and/or at different quantities in a recipe.
     * To be clear, it is preferable to list “1 1/2 cups of sugar” and then “1/2 cup of sugar” (as specified below) than to list “2 cups sugar, divided”. */
    pub ingredients: Option<Vec<Ingredient>>,

    /* This is a field that will appear in several locations. The recipe itself may have notes, each ingredient may have notes, and each step may have notes. */
    pub notes: Option<String>,

    /* If this recipe was originally pulled from a book, then the book information should go here.
     * Recipe software should make an intelligent effort to include correct information in the correct fields,
     * rather than just dumping everything into a generic notes field. */
    pub source_book: Option<BookSource>,

    /* Does not refer to the person who entered the recipe; only refers to the original author of the recipe.
     * If this recipe was based on another recipe by another person, then this field should contain the name of the original author. */
    pub source_authors: Option<Vec<String>>,

    /* The URL that this recipe was copied from, if applicable. In the case of a recipe-hosting website, this may refer to the official URL at which the recipe is hosted. */
    pub source_url: Option<String>,

    /* A list, in order, of steps to be performed on the recipe. Each item in the list is a dict, as specified below. */
    pub steps: Option<Vec<Step>>,

    /* Refers to how much food the recipe makes. This is a list, which will normally contain one dict.
     * In cases where multiple yields need to be stored (i.e. 50 cookies vs 100 cookes vs 250 cookies), each yield will have its own dict in this list. */
    pub yields: Option<Vec<Yield>>,
}

impl Recipe {
    pub fn new(uid: &str) -> Self {
        Self {
            recipe_uuid: uid.into(),
            recipe_name: None,
            oven_fan: None,
            oven_temp: None,
            oven_time: None,
            ingredients: None,
            notes: None,
            source_book: None,
            source_authors: None,
            source_url: None,
            steps: None,
            yields: None,
        }
    }
}
