use serde::{Deserialize, Serialize};
use yew::agent::{Dispatched, Dispatcher};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::app::{RouteServiceType, RouteType};
use crate::reroute_agent::{RerouteAgent, RerouteRequestMsg};

// Struct for making add recipe requests
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeRequest {
    pub recipe_name: Option<String>,
    pub oven_time: Option<f64>,
    pub notes: Option<String>,
    pub oven_fan: Option<OvenFanValue>,
}

impl RecipeRequest {
    pub fn valid(&self) -> bool {
        self.recipe_name_valid()
    }

    fn recipe_name_valid(&self) -> bool {
        match &self.recipe_name {
            None => false,
            Some(name) => name.chars().count() >= 4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    amount: f32,
    unit: TemperatureUnit,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    amount: f32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    data: IngredientData,

    /* This field is a list of ingredients, in exactly the same format as a regular ingredient list item, minus the substitutions field.
     * For instance, it must contain amounts, and may also contain processing, usda_num, notes, etc. */
    substitutions: Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct HACCPValue {
    /* Refers to specific HACCP guidelines relevant to this step. */
    control_point: String,

    /* Refers to specific HACCP guidelines relevant to this step, which are critical to the safety outcome of this recipe.
     * For instance, “Cook until the food reaches an internal temperature of 165F.” */
    critical_control_point: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    /* The only item in the dict that is absolutely required. */
    step: String,

    /* A dict, which can contain either a control_point or a critical_control_point. Should not contain both. */
    haccp: Option<HACCPValue>,

    /* A list of notes relevant to this step. Often known as “bench notes” to professionals. */
    notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Yield {
    /* The amount, relevant to the unit. */
    amount: f32,

    /* Generally “servings”, but up to the user. Can be “packages”, “cups”, “glasses”, etc. */
    unit: String,
}

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
    oven_fan: Option<OvenFanValue>,

    /* Starting oven temperature, if the oven is used. */
    oven_temp: Option<Temperature>,

    /* How long the dish should spend in the oven.
     * This is an overall value, which refers to the recipe as a whole. If multiple oven times are used, they should be specified in the recipe. */
    oven_time: Option<f64>,

    /* A list of dicts, defining which food items are to be added to the recipe. These items should be listed in the order in which they are to be used.
     * Bearing this in mind, a particular item may be listed multiple times, if it is to be used multiple times and/or at different quantities in a recipe.
     * To be clear, it is preferable to list “1 1/2 cups of sugar” and then “1/2 cup of sugar” (as specified below) than to list “2 cups sugar, divided”. */
    ingredients: Option<Vec<Ingredient>>,

    /* This is a field that will appear in several locations. The recipe itself may have notes, each ingredient may have notes, and each step may have notes. */
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

pub enum Msg {
    GetRecipe,
    ReceiveFetchRecipeResponse(Result<Recipe, anyhow::Error>),
    BackToAllRecipes,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub recipe_uuid: String,
}

pub struct RecipeComp {
    link: ComponentLink<Self>,
    reroute_agent: Dispatcher<RerouteAgent>,
    model: Recipe,
    fetch_recipe_task: Option<FetchTask>,
    fetch_error_msg: Option<String>,
}

impl Component for RecipeComp {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let recipe = Recipe::new(&props.recipe_uuid);

        let mut recipe_comp = Self {
            link,
            model: recipe,
            fetch_recipe_task: None,
            fetch_error_msg: None,
            reroute_agent: RerouteAgent::dispatcher(),
        };

        recipe_comp.fetch_recipe();

        recipe_comp
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetRecipe => {
                self.fetch_recipe();

                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            Msg::ReceiveFetchRecipeResponse(response) => {
                match response {
                    Ok(recipe) => {
                        self.model = recipe;
                        self.fetch_error_msg = Some("".to_string());
                    }
                    Err(error) => {
                        self.fetch_error_msg = Some(error.to_string());
                    }
                }
                self.fetch_recipe_task = None;

                // we want to redraw so that the page displays the fetched recipes instead of 'fetching...'
                true
            }
            Msg::BackToAllRecipes => {
                let new_route = String::from("/recipes/");
                self.reroute_agent
                    .send(RerouteRequestMsg::Reroute(new_route));

                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {<>
            { self.view_fetching() }
            { self.view_error() }

            { self.view_back_to_recipes() }

            <br/>
            <h2>{"Recipe"}</h2>

            <h3>{"recipe_uuid"}</h3>
            <p>{ &self.model.recipe_uuid }</p>

            <h3>{"recipe_name"}</h3>
            <p>{ match &self.model.recipe_name {
                Some(name) => name,
                None => "",
            }}</p>

            <h3>{"oven_time"}</h3>
            <p>{ match &self.model.oven_time {
                Some(t) => t.to_string(),
                None => "Null".to_string(),
            }}</p>

            <h3>{"notes"}</h3>
            <p>{ match &self.model.notes {
                Some(s) => s.clone(),
                None => "Null".to_string(),
            }}</p>

            <h3>{"oven_fan"}</h3>
            <p>{ OvenFanValue::to_string(&self.model.oven_fan) }</p>

        </>}
    }
}

impl RecipeComp {
    fn build_fetch_recipe_task(recipe_uuid: &str, link: &ComponentLink<Self>) -> FetchTask {
        // 1. build the request
        let request = Request::get(format!("http://localhost:8080/recipe/{}", &recipe_uuid))
            .body(Nothing)
            .expect("Could not build request.");

        // 2. construct a callback
        let callback = link.callback(|response: Response<Json<Result<Recipe, anyhow::Error>>>| {
            let Json(data) = response.into_body();
            Msg::ReceiveFetchRecipeResponse(data)
        });

        // 3. pass the request and callback to the fetch service
        let task = FetchService::fetch(request, callback).expect("failed to start request");

        task
    }

    fn fetch_recipe(&mut self) {
        // 4. store the task so it isn't canceled immediately
        self.fetch_recipe_task = Some(RecipeComp::build_fetch_recipe_task(
            &self.model.recipe_uuid,
            &self.link,
        ));
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_recipe_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.fetch_error_msg {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }

    fn view_back_to_recipes(&self) -> Html {
        html! {

            <button class="ui labeled icon button",
                    onclick=self.link.callback(|_| Msg::BackToAllRecipes),
                    >
                <i class="left chevron icon"></i>
                { "Back to Recipes" }
            </button>
        }
    }
}
