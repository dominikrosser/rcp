use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use serde::Deserialize;
use serde_json::json;

pub struct AddRecipeComp {
    link: ComponentLink<Self>,
    state: State,
}

pub struct State {
    post_recipes_task: Option<FetchTask>,
    post_response_display_msg: Option<String>,
}

impl State {
    fn new() -> Self {
        State {
            post_recipes_task: None,
            post_response_display_msg: None,
        }
    }
}

pub enum Msg {
    Noop,
    AddRecipe,
    ReceiveSuccessResponse,
    ReceiveErrorResponse,
}

impl Component for AddRecipeComp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::new();

        Self {
            link,
            state,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => { false },
            Msg::AddRecipe => {
                let json_value: serde_json::Value = json!({"recipe_name": "NEW RECIPE"});
                let json_body = Json(&json_value);

                // 1. build the request
                let post_request = Request::post("http://localhost:8080/recipe")
                    .header("content-type", "application/json")
                    .body(json_body)
                    .expect("Could not build that request.");

                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Result<String, anyhow::Error>>| {
                            if response.status().is_success() {
                                Msg::ReceiveSuccessResponse
                            } else {
                                Msg::ReceiveErrorResponse
                            }
                        });
                
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(post_request, callback).expect("failed to start post request");
                
                // 4. store the task so it isn't canceled immediately
                self.state.post_recipes_task = Some(task);

                false
            },
            Msg::ReceiveSuccessResponse => {
                self.state.post_response_display_msg = Some("Success".to_string());
                true
            },
            Msg::ReceiveErrorResponse => {
                self.state.post_response_display_msg = Some("Error".to_string());
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {<>
            <h2>{"Add Recipe"}</h2>
            { self.view_submit_recipe_button() }
        </>}
    }
}

impl AddRecipeComp {
    fn view_submit_recipe_button(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::AddRecipe)>
                { "Submit" }
            </button>
        }
    }
}