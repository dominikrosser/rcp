use serde::{Deserialize, Serialize};
use yew::agent::{Dispatched, Dispatcher};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use rcp_shared_rs_code::models::book_source::BookSource;
use rcp_shared_rs_code::models::haccp_value::HACCPValue;
use rcp_shared_rs_code::models::ingredient::Ingredient;
use rcp_shared_rs_code::models::oven_fan_value::OvenFanValue;
use rcp_shared_rs_code::models::r#yield::Yield;
use rcp_shared_rs_code::models::step::Step;
use rcp_shared_rs_code::models::temperature::Temperature;
use rcp_shared_rs_code::models::temperature_unit::TemperatureUnit;

use rcp_shared_rs_code::models::recipe::Recipe;

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
        let recipe = Recipe { recipe_uuid: props.recipe_uuid, ..Default::default() };

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
            <p>{ if let Some(of) = &self.model.oven_fan {
                OvenFanValue::to_string(of)
                } else {
                    "".to_string()
                }
            }</p>

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
