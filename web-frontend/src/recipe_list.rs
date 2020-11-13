use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use serde::Deserialize;

use crate::recipe::Recipe;

pub struct RecipeList {
    link: ComponentLink<Self>,
    state: State
}

pub struct State {
    recipes: Option<Vec<Recipe>>,
    fetch_recipes_task: Option<FetchTask>,
    fetch_error_msg: Option<String>,
}

pub enum Msg {
    GetRecipes,
    ReceiveFetchRecipesResponse(Result<Vec<Recipe>, anyhow::Error>),
}

impl Component for RecipeList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            recipes: None,// Some(State::dummies(500)),
            fetch_recipes_task: None,
            fetch_error_msg: None,
        };

        RecipeList {
            link,
            state,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetRecipes => {

                // 1. build the request
                let request = Request::get("http://localhost:8080/recipe")
                    .body(Nothing)
                    .expect("Could not build request.");

                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Vec<Recipe>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveFetchRecipesResponse(data)
                        });
                
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                // 4. store the task so it isn't canceled immediately
                self.state.fetch_recipes_task = Some(task);

                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            },
            Msg::ReceiveFetchRecipesResponse(response) => {
                match response {
                    Ok(recipes) => {
                        self.state.recipes = Some(recipes);
                        self.state.fetch_error_msg = Some("".to_string());
                    }
                    Err(error) => {
                        self.state.fetch_error_msg = Some(error.to_string());
                    }
                }
                self.state.fetch_recipes_task = None;

                // we want to redraw so that the page displays the fetched recipes instead of 'fetching...'
                true
            },
        }
    }

    fn view(&self) -> Html {
        html! {<>
            { self.view_fetch_recipes_button() }
            { self.view_fetching() }
            { self.view_recipe_list() }
            // <h2>{"Recipe List"}</h2>
            // <table class="ui celled padded table">
            //     <thead>
            //         <tr>
            //             <th class="single line">{"Recipe Name"}</th>
            //         </tr>
            //     </thead>
            //     <tbody>
            //         {
            //             self.view_recipe_list()
            //             // for self.state.recipes
            //             //     .iter()
            //             //     .enumerate()
            //             //     .map(|entry| self.view_entry(entry))
            //         }
            //     </tbody>
            // </table>
            { self.view_error() }
        </>}
    }
}

impl RecipeList {
    fn view_fetch_recipes_button(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::GetRecipes)>
                { "Fetch Recipes" }
            </button>
        }
    }

    fn view_recipe_list(&self) -> Html {
        if let Some(ref recipes) = self.state.recipes {
            let recipes_html = html! {
                for recipes
                    .iter()
                    .enumerate()
                    .map(|entry| self.view_entry(entry)) 
            };

            let recipe_list_html = html! {<>
                <h2>{"Recipe List"}</h2>
                <table class="ui celled padded table">
                    <thead>
                        <tr>
                            <th class="single line">{"Recipe Name"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { recipes_html }
                    </tbody>
                </table>
            </>};

            recipe_list_html

        } else {
            html! { "Please fetch recipes" }
        }
    }

    fn view_entry(&self, (idx, recipe): (usize, &Recipe)) -> Html {
        html!{
            <tr>
                <td>
                    {
                        if let Some(ref name) = recipe.recipe_name {
                            name.clone()
                        } else {
                            "".to_string()
                        }
                    }
                </td>
            </tr>
        }
    }

    fn view_fetching(&self) -> Html {
        if self.state.fetch_recipes_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.state.fetch_error_msg {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl State {
    fn dummies(size: u32) -> Vec<Recipe> {
        let mut zero_vec: Vec<Recipe> = Vec::with_capacity(size as usize);
        for i in 0..size {
            zero_vec.push(Recipe::new(""));
        }
        return zero_vec;
    }
}