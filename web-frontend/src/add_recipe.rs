use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::agent::{Dispatched, Dispatcher};
use yew::callback::Callback;
use yew::events::ChangeData;
use yew::events::MouseEvent;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::{route::Route, service::RouteService, Switch};
use std::str::FromStr;

use rcp_shared_rs_code::models::oven_fan_value::OvenFanValue;
use rcp_shared_rs_code::models::temperature::Temperature;
use rcp_shared_rs_code::models::temperature_unit::TemperatureUnit;
use rcp_shared_rs_code::models::recipe_request::RecipeRequest;

use crate::app::RouteServiceType;
use crate::app::RouteType;
use crate::reroute_agent::{RerouteAgent, RerouteRequestMsg};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecipeResponse {
    pub status: u16,
    pub recipe_uuid: String,
}

pub struct AddRecipeComp {
    link: ComponentLink<Self>,
    state: State,
    reroute_agent: Dispatcher<RerouteAgent>,
}

pub struct State {
    recipe_data: RecipeRequest,
    post_recipes_task: Option<FetchTask>,
    post_response_display_msg: Option<String>, // TODO: Vector with multiple messages
}

impl State {
    fn new() -> Self {
        let recipe_data: RecipeRequest = Default::default();

        State {
            post_recipes_task: None,
            post_response_display_msg: None,
            recipe_data: recipe_data,
        }
    }
}

pub enum Msg {
    Noop,
    AddRecipe,
    ReceivePostResponse(Result<CreateRecipeResponse, anyhow::Error>),

    RecipeNameInputChanged(String),
    RecipeOvenTimeInputChanged(String),
    RecipeNotesInputChanged(String),
    RecipeOvenFanSelectChanged(String),
    RecipeOvenTempAmountInputChanged(String),//TODO
    RecipeOvenTempUnitInputChanged(String),//TODO
    // SOURCE BOOK
    // SOURCE AUTHORS
    RecipeSourceUrlInputChanged(String),//TODO
    // INGREDIENTS
    OnAddIngredients,
    // STEPS
    OnAddSteps,
    // YIELDS
    OnAddYields,
}

impl Component for AddRecipeComp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::new();

        Self {
            link,
            state,
            reroute_agent: RerouteAgent::dispatcher(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => false,
            Msg::AddRecipe => {
                let task: FetchTask = self.build_fetch_recipe_task();

                // 4. store the task so it isn't canceled immediately
                self.state.post_recipes_task = Some(task);

                true
            },
            Msg::ReceivePostResponse(data) => {
                self.state.recipe_data.recipe_name = None;
                self.state.recipe_data.oven_time = None;
                self.state.recipe_data.oven_fan = None;
                self.state.post_recipes_task = None;

                match data {
                    Ok(recipe_response) => {
                        let new_route = format!("/recipes/{}", recipe_response.recipe_uuid);
                        self.reroute_agent
                            .send(RerouteRequestMsg::Reroute(new_route));
                        self.state.post_response_display_msg =
                            Some("Successfully added recipe".to_string());
                    }
                    Err(err) => {
                        self.state.post_response_display_msg =
                            Some("Error adding recipe".to_string());
                    }
                }

                true
            },
            Msg::RecipeNameInputChanged(recipe_name) => {
                self.state.recipe_data.recipe_name = Some(recipe_name);
                true
            },
            Msg::RecipeOvenTimeInputChanged(oven_time) => {
                let oven_time: f64 = oven_time.parse::<f64>().unwrap();
                self.state.recipe_data.oven_time = Some(oven_time);
                true
            },
            Msg::RecipeNotesInputChanged(notes) => {
                self.state.recipe_data.notes = Some(notes);
                true
            },
            Msg::RecipeOvenFanSelectChanged(oven_fan) => {
                self.state.recipe_data.oven_fan = OvenFanValue::from_str(&oven_fan).ok();
                true
            },
            Msg::RecipeOvenTempAmountInputChanged(amount_str) => {
                let amount: f64 = amount_str.parse().unwrap_or(0.0f64);
                self.state.recipe_data.oven_temp = self.state.recipe_data.oven_temp.as_ref().map_or(None, |temp| Some(Temperature { amount: amount, unit: temp.unit.clone() }));
                true
            },
            Msg::RecipeOvenTempUnitInputChanged(unit_str) => {
                let unit = TemperatureUnit::from_str(&unit_str).unwrap();
                self.state.recipe_data.oven_temp = self.state.recipe_data.oven_temp.as_ref().map_or(None, |temp| Some(Temperature { amount: temp.amount, unit: unit }));
                true
            },
            Msg::RecipeSourceUrlInputChanged(source_url_str) => {
                self.state.recipe_data.source_url = Some(source_url_str);
                true
            },
            Msg::OnAddYields => {
                self.state.recipe_data.yields = Some(Default::default());
                true
            },
            Msg::OnAddSteps => {
                self.state.recipe_data.steps = Some(Default::default());
                true
            },
            Msg::OnAddIngredients => {
                self.state.recipe_data.ingredients = Some(Default::default());
                true
            },
        }
    }

    fn view(&self) -> Html {
        if self.state.post_recipes_task.is_some() {
            html! {<div class="ui medium text loader active">{ "Uploading..."}</div>}
        } else {
            let oninput: Callback<InputData> = self
                .link
                .callback(|e: InputData| Msg::RecipeNameInputChanged(e.value));

            html! {<>
                {
                    if let Some(msg) = &self.state.post_response_display_msg {
                        html!{ <p>{msg}</p>}
                    } else {
                        html!{}
                    }
                }
                <br/>
                <h2>{"Add Recipe"}</h2>

                <form class="ui form">
                    { self.view_recipe_name_input() }
                    { self.view_oven_time_input() }
                    { self.view_notes_input() }
                    { self.view_oven_fan_input() }
                    { self.view_steps_input() }
                    { self.view_yields_input() }
                    { self.view_ingredients_input() }
                    { self.view_oven_temp_input() }
                    { self.view_submit_recipe_button() }
                </form>

            </>}
        }
    }
}

impl AddRecipeComp {
    fn build_fetch_recipe_task(&self) -> FetchTask {
        let json_body = Json(&self.state.recipe_data);

        // 1. build the request
        let post_request = Request::post("http://localhost:8080/recipe")
            .header("content-type", "application/json")
            .body(json_body)
            .expect("Could not build that request.");

        // 2. construct a callback
        let callback = self.link.callback(
            |response: Response<Json<Result<CreateRecipeResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceivePostResponse(data)
            },
        );

        // 3. pass the request and callback to the fetch service
        let task =
            FetchService::fetch(post_request, callback).expect("failed to start post request");

        task
    }

    fn view_submit_recipe_button(&self) -> Html {
        html! {
            <button class="ui button" type="submit" onclick=self.link.callback(|_| Msg::AddRecipe)>
                { "Submit" }
            </button>
        }
    }

    fn view_oven_temp_input(&self) -> Html {
        match &self.state.recipe_data.oven_temp {
            None => html!{},
            Some(temp) => {
                let on_oven_temp_amount_input = self.link.callback(|e: InputData| {
                        Msg::RecipeOvenTempAmountInputChanged(e.value)
                    });

                html! {
                    
                    <div class="field">
                        <label for="oven_temp_amount_input">{"oven_temp_amount: "}</label>
                        <input
                            type="number"
                            id="oven_temp_amount_input"
                            value=temp.amount,
                            oninput=on_oven_temp_amount_input,
                            />

                    </div>
                }
            }
        }
    }

    fn view_oven_fan_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="oven_fan_select">{"oven_fan: "}</label>
                <select
                    name="oven_fan",
                    id="oven_fan_select",
                    value={if let Some(of) = &self.state.recipe_data.oven_fan { OvenFanValue::to_string(of) } else { "".to_string() }},
                    onchange=self.link.callback(|e: ChangeData| Msg::RecipeOvenFanSelectChanged(match e {
                        ChangeData::Select(selElement) => selElement.value(),
                        _ => "".to_string(),
                    }))
                    >
                    <option value="">{"-"}</option>
                    <option value="Off">{"Off"}</option>
                    <option value="Low">{"Low"}</option>
                    <option value="High">{"High"}</option>
                </select>
            </div>
        }
    }

    fn view_notes_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="notes_input">{"notes: "}</label>
                <textarea
                    rows=4,
                    type="text",
                    id="notes_input",
                    value=match &self.state.recipe_data.notes {
                        None => "",
                        Some(s) => s,
                    },
                    oninput=self.link.callback(|e: InputData| Msg::RecipeNotesInputChanged(e.value))
                    />
            </div>
        }
    }

    fn view_oven_time_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="oven_time_input">{"oven_time: "}</label>
                <input
                    type="number"
                    id="oven_time_input"
                    value=match &self.state.recipe_data.oven_time {
                        None => "".to_string(),
                        Some(t) => t.to_string(),
                    },
                    oninput=self.link.callback(|e: InputData| Msg::RecipeOvenTimeInputChanged(e.value))
                    />
            </div>
        }
    }

    fn view_recipe_name_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="recipe_name_input">{"recipe_name: "}</label>
                <input
                    type="text",
                    id="recipe_name_input"
                    value=match &self.state.recipe_data.recipe_name {
                        None => "",
                        Some(name) => name,
                    },
                    oninput=self.link.callback(|e: InputData| Msg::RecipeNameInputChanged(e.value))
                    />
            </div>
        }
    }
    
    fn view_steps_input(&self) -> Html {
        if self.state.recipe_data.steps.is_some() {
            html! {

            }
        } else {
            html! {
                { self.view_add_btn("Add Steps", |_| Msg::OnAddSteps) }
            }
        }
    }

    fn view_ingredients_input(&self) -> Html {
        if self.state.recipe_data.ingredients.is_some() {
            html! {
                
            }
        } else {
            html! {
                { self.view_add_btn("Add Ingredients", |_| Msg::OnAddIngredients) }
            }
        }
    }

    fn view_add_btn<P: 'static>(&self, text: &str, cb: P) -> Html where
        P: Fn(MouseEvent) -> Msg {
        html! {
            <button class="ui teal labeled icon button" onclick=self.link.callback(cb)>
                {text}
                <i class="add icon"></i>
            </button>
        }
    }

    fn view_yields_input(&self) -> Html {
        if self.state.recipe_data.yields.is_some() {
            html! {}
        } else {
            html! {
                { self.view_add_btn("Add Yields", |_| Msg::OnAddYields) }
            }
        }
    }
}
