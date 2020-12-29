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
        let recipe = Recipe {
            recipe_uuid: props.recipe_uuid,
            ..Default::default()
        };

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
            <h2 class="ui header">{ self.model.recipe_name.as_ref().unwrap_or(&"Recipe".to_string()) }</h2>


            { self.view_notes() }

            <h3 class="ui header">{"Oven"}</h3>
            { self.view_oven_fan() }
            { self.view_oven_time() }

            { self.view_ingredients() }

            { self.view_yields() }
            { self.view_steps() }

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

    fn view_notes(&self) -> Html {
        self.model.notes.as_ref().map_or(html! {}, |s| {
            html! {<>
                <h3 class="ui header">{"Notes"}</h3>
                <p>{ s.clone() }</p>
            </>}
        })
    }

    fn view_yields(&self) -> Html {
        self.model.yields.as_ref().map_or(html! {}, |yields| {
            let yields_html = html! {
                for yields
                    .iter()
                    .enumerate()
                    .map(|(pos, entry)| self.view_yield((pos, entry)))
            };
            html! {<>
                <h3 class="ui header">{"Yields"}</h3>
                <ul>
                    { yields_html }
                </ul>
            </>}
        })
    }

    fn view_yield(&self, (idx, r#yield): (usize, &Yield)) -> Html {
        html! {
            <li>
                { format!("{} {}", r#yield.amount, r#yield.unit) }
            </li>
        }
    }

    fn view_oven_fan(&self) -> Html {
        self.model.oven_fan.as_ref().map_or(html! {}, |of| {
            html! {
                <p>{ format!("Fan: {}", OvenFanValue::to_string(of)) }</p>
            }
        })
    }

    fn view_oven_time(&self) -> Html {
        self.model.oven_time.as_ref().map_or(html! {}, |ot| {
            html! {
                <p>{ format!("Total time: {}", ot) }</p>
            }
        })
    }

    fn view_steps(&self) -> Html {
        self.model.steps.as_ref().map_or(html! {}, |steps| {
            if steps.is_empty() {
                html! {}
            } else {
                let steps_html = html! {
                    for steps
                        .iter()
                        .enumerate()
                        .map(|(pos, entry)| self.view_step((pos, entry)))
                };

                html! {<>
                    <h3 class="ui header">{"Steps"}</h3>
                    <div class="ui raised segments">
                        { steps_html }
                    </div>
                </>}
            }
        })
    }

    fn view_step(&self, (idx, step): (usize, &Step)) -> Html {
        let notes_html = step.notes.as_ref().map_or(html! {}, |s| html! {{s}});

        html! {
            <div class="ui segment">
                <div>
                    <p><b>{ idx + 1 }</b></p>
                </div>
                <div class="content">
                    <p>{ &step.step }</p>
                    <p><em>{ notes_html }</em></p>
                </div>
            </div>
        }
    }

    fn view_ingredients(&self) -> Html {
        self.model
            .ingredients
            .as_ref()
            .map_or(html! {}, |ingredients| {
                if ingredients.is_empty() {
                    html! {}
                } else {
                    let ingredients_html = html! {
                        for ingredients
                            .iter()
                            .enumerate()
                            .map(|(pos, entry)| self.view_ingredient((pos, entry)))
                    };

                    html! {<>
                        <h3 class="ui header">{"Ingredients"}</h3>
                        <div class="ui list">
                            { ingredients_html }
                        </div>
                    </>}
                }
            })
    }

    fn view_ingredient(&self, (idx, ing): (usize, &Ingredient)) -> Html {
        let cb_id = format!("ingredient-checkbox-{}", idx);
        let i = &ing.ingredient;

        html! {
            <div class="item">
                <div class="ui checkbox">
                    <input type="checkbox" id=&cb_id />
                    <label for=&cb_id>{&i.ingredient_name}</label>
                </div>
            </div>
        }
    }
}
