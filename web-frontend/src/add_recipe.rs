use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::agent::{Dispatched, Dispatcher};
use yew::callback::Callback;
use yew::events::ChangeData;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::{route::Route, service::RouteService, Switch};

use rcp_shared_rs_code::models::oven_fan_value::OvenFanValue;

use crate::app::RouteServiceType;
use crate::app::RouteType;
use crate::recipe::RecipeRequest;
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
        let recipe_data: RecipeRequest = RecipeRequest {
            recipe_name: None,
            oven_time: None,
            oven_fan: None,
            notes: None,
        };

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
            }
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
            }

            Msg::RecipeNameInputChanged(recipe_name) => {
                self.state.recipe_data.recipe_name = Some(recipe_name);
                true
            }
            Msg::RecipeOvenTimeInputChanged(oven_time) => {
                let oven_time: f64 = oven_time.parse::<f64>().unwrap();
                self.state.recipe_data.oven_time = Some(oven_time);
                true
            }
            Msg::RecipeNotesInputChanged(notes) => {
                self.state.recipe_data.notes = Some(notes);
                true
            }
            Msg::RecipeOvenFanSelectChanged(oven_fan) => {
                self.state.recipe_data.oven_fan = OvenFanValue::from_string(&oven_fan);
                true
            }
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

                    <div class="field">
                        <label for="oven_fan_select">{"oven_fan: "}</label>
                        <select
                            name="oven_fan",
                            id="oven_fan_select",
                            value=OvenFanValue::to_string(&self.state.recipe_data.oven_fan),
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

                    { self.view_submit_recipe_button() }
                </form>

            </>}
        }
    }
}

impl AddRecipeComp {
    fn build_fetch_recipe_task(&self) -> FetchTask {
        // let json_value: serde_json::Value = json!({"recipe_name": &self.state.recipe_data.recipe_name });
        // let json_body = Json(&json_value);
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
}
