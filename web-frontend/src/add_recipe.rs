use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::callback::Callback;
use yew::events::ChangeData;
use yew::format::{Json, Nothing};
use serde::{Serialize, Deserialize};
use serde_json::json;
use yew_router::{route::Route, service::RouteService, Switch};
use yew::agent::{Dispatched, Dispatcher};

use crate::recipe::RecipeRequest;
use crate::app::RouteType;
use crate::app::RouteServiceType;
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
    post_response_display_msg: Option<String>,// TODO: Vector with multiple messages
}

impl State {
    fn new() -> Self {
        let recipe_data: RecipeRequest = RecipeRequest {
            recipe_name: None,
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
}

impl Component for AddRecipeComp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::new();

        Self {
            link,
            state,
            reroute_agent: RerouteAgent::dispatcher()
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => { false },
            Msg::AddRecipe => {
                
                let task: FetchTask = self.build_fetch_recipe_task();

                // 4. store the task so it isn't canceled immediately
                self.state.post_recipes_task = Some(task);

                true
            },
            Msg::ReceivePostResponse(data) => {
                self.state.post_response_display_msg = Some("Successfully added recipe".to_string());

                // let route_string = format!("/recipe/");
                // let mut route_service: RouteService<()> = RouteService::new();
                // route_service.set_route(&route_string, ());

                // self.route = Route {
                //     route: route_string,
                //     state: (),
                // };
                
                self.state.recipe_data.recipe_name = None;
                self.state.post_recipes_task = None;
                
                match data {
                    Ok(recipe_response) => {
                        let new_route = format!("/recipes/{}", recipe_response.recipe_uuid);
                        self.reroute_agent
                            .send(RerouteRequestMsg::Reroute(new_route));
                    },
                    Err(err) => {}
                }

                true
            },
            Msg::RecipeNameInputChanged(recipe_name) => {
                self.state.recipe_data.recipe_name = Some(recipe_name);
                true
            },
        }
    }

    fn view(&self) -> Html {

        if self.state.post_recipes_task.is_some() {
            html!{<div class="ui medium text loader active">{ "Uploading..."}</div>}
        } else {
            let oninput = self.link.callback(|e: InputData| {
                    Msg::RecipeNameInputChanged(e.value)
                });

            html! {<>
                { 
                    if let Some(msg) = &self.state.post_response_display_msg {
                        html!{ <p>{msg}</p>}
                    } else {
                        html!{}
                    }
                }
                <h2>{"Add Recipe"}</h2>
                
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
                { self.view_submit_recipe_button() }
            </>}
        }
    }
}

impl AddRecipeComp {

    fn build_fetch_recipe_task(&self) -> FetchTask {
        let json_value: serde_json::Value = json!({"recipe_name": &self.state.recipe_data.recipe_name });
        let json_body = Json(&json_value);

        // 1. build the request
        let post_request = Request::post("http://localhost:8080/recipe")
            .header("content-type", "application/json")
            .body(json_body)
            .expect("Could not build that request.");

        // 2. construct a callback
        let callback =
            self.link
                .callback(|response: Response<Json<Result<CreateRecipeResponse, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceivePostResponse(data)
                });
        
        // 3. pass the request and callback to the fetch service
        let task = FetchService::fetch(post_request, callback).expect("failed to start post request");

        task
    }

    fn view_submit_recipe_button(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::AddRecipe)>
                { "Submit" }
            </button>
        }
    }
}