use yew::prelude::*;
use serde::{Deserialize};

pub struct RecipeComp {
    link: ComponentLink<Self>,
    model: Recipe,
}

#[derive(Deserialize)]
pub struct Temperature {
    amount: f32,
    unit: TemperatureUnit,
}

#[derive(Deserialize)]
pub enum OvenFanValue {
    Off,
    Low,
    High
}

#[derive(Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Deserialize)]
pub struct Amount {
    amount: f32,
    unit: String,
}

#[derive(Deserialize)]
pub struct IngredientData {
    /* A list of dicts which describe the amounts to use. Normally, the list will only contain one dict.
     * In cases where multiple yields need to be stored (i.e. 50 cookies vs 100 cookes vs 250 cookies),
     * each yield will have its own dict in this list, in the same order as the recipe’s yield field. */
    amounts: Vec<Amount>,

    /* A list of tags which describe the processing of this item. For instance, “whole”, “large dice”, “minced”, “raw”, “steamed”, etc. */
    processing: Vec<String>,
 
    /* Any notes specific to this ingredient. */
    notes: String,

    /* This corresponds with the index keys in the USDA Standard Reference. It is generally used for easy lookup of nutritional data.
     * If possible, this should be used, and USDA data, when available, is preferable to any other nutritional data source. */
    usda_num: Option<String>,
}

// A dict of items, describing an ingredient, and how much of that ingredient to use.
#[derive(Deserialize)]
pub struct Ingredient {

    data: IngredientData,

    /* This field is a list of ingredients, in exactly the same format as a regular ingredient list item, minus the substitutions field.
     * For instance, it must contain amounts, and may also contain processing, usda_num, notes, etc. */
    substitutions: Vec<Ingredient>,
}

#[derive(Deserialize)]
pub struct BookSource {
    /* This is a list. Refers to the author(s) of this recipe. Can be the same as source_authors, if appropriate.
     * If there was only one author, then they would be the only item in the list. */
    authors: Vec<String>,

    /* Title of the book. This is a single value, not a list. */
    title: String,

    /* International Standard Book Number, if available. */
    isbn: Option<String>,

    /* Any information about the book that does not fit into another field. */
    notes: Option<String>,
}

#[derive(Deserialize)]
pub struct HACCPValue {
    /* Refers to specific HACCP guidelines relevant to this step. */
    control_point: String,

    /* Refers to specific HACCP guidelines relevant to this step, which are critical to the safety outcome of this recipe.
     * For instance, “Cook until the food reaches an internal temperature of 165F.” */
    critical_control_point: String,
}

#[derive(Deserialize)]
pub struct Step {
    /* The only item in the dict that is absolutely required. */
    step: String,

    /* A dict, which can contain either a control_point or a critical_control_point. Should not contain both. */
    haccp: Option<HACCPValue>,

    /* A list of notes relevant to this step. Often known as “bench notes” to professionals. */
    notes: Option<String>,
}

#[derive(Deserialize)]
pub struct Yield {
    /* The amount, relevant to the unit. */
    amount: f32,

    /* Generally “servings”, but up to the user. Can be “packages”, “cups”, “glasses”, etc. */
    unit: String,
}

// See Open Recipe Format
#[derive(Deserialize)]
pub struct Recipe {

    // recipe_uuid
    recipe_uuid: String,

    /* The name of this recipe. */
    pub recipe_name: Option<String>,

    /* Setting to be used with convection oven. Possible values are “Off”, “Low” and “High”. If not specified, it is assumed to be “Off”.
     * If specified, all software should display and print this value. If not specified, it is up to the software whether or not it is displayed and/or printed,
     * but it should be consistent. */
    oven_fan: Option<OvenFanValue>,

    /* Starting oven temperature, if the oven is used. */
    oven_temp: Option<Temperature>,

    /* How long the dish should spend in the oven.
     * This is an overall value, which refers to the recipe as a whole. If multiple oven times are used, they should be specified in the recipe. */
    oven_time: Option<f32>,

    /* A list of dicts, defining which food items are to be added to the recipe. These items should be listed in the order in which they are to be used.
     * Bearing this in mind, a particular item may be listed multiple times, if it is to be used multiple times and/or at different quantities in a recipe.
     * To be clear, it is preferable to list “1 1/2 cups of sugar” and then “1/2 cup of sugar” (as specified below) than to list “2 cups sugar, divided”. */
    ingredients: Option<Vec<Ingredient>>,

    /* This is a field that will appear in several locations. The recipe itself may have noted, each ingredient may have notes, and each step may have notes. */
    notes: Option<String>,

    /* If this recipe was originally pulled from a book, then the book information should go here.
     * Recipe software should make an intelligent effort to include correct information in the correct fields,
     * rather than just dumping everything into a generic notes field. */
    source_book: Option<BookSource>,

    /* Does not refer to the person who entered the recipe; only refers to the original author of the recipe.
     * If this recipe was based on another recipe by another person, then this field should contain the name of the original author. */
    source_authors: Option<Vec<String>>,

    /* The URL that this recipe was copied from, if applicable. In the case of a recipe-hosting website, this may refer to the official URL at which the recipe is hosted. */
    source_url: Option<String>,

    /* A list, in order, of steps to be performed on the recipe. Each item in the list is a dict, as specified below. */
    steps: Option<Vec<Step>>,

    /* Refers to how much food the recipe makes. This is a list, which will normally contain one dict.
     * In cases where multiple yields need to be stored (i.e. 50 cookies vs 100 cookes vs 250 cookies), each yield will have its own dict in this list. */
    yields: Option<Vec<Yield>>,
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

pub enum Msg {}

impl Component for RecipeComp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let recipe = Recipe::new("".into());

        Self {
            link,
            model: recipe,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
        }
    }

    fn view(&self) -> Html {
        html! {
            <h2>{"Recipe"}</h2>
        }
    }
}