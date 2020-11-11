use yew::prelude::*;

use super::recipe;

pub struct RecipeList {
    link: ComponentLink<Self>,
    state: State
}

pub struct State {
    recipes: Vec<recipe::Recipe>,
}

pub enum Msg {}

impl Component for RecipeList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            recipes: State::dummies(500),
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
        }
    }

    fn view(&self) -> Html {
        html! {<>
            <h2>{"Recipe List"}</h2>
            <table class="ui celled padded table">
                <thead>
                    <tr>
                        <th class="single line">{"Recipe Name"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for self.state.recipes
                        .iter()
                        .enumerate()
                        .map(|entry| self.view_entry(entry)) }
                </tbody>
            </table>
        </>}
    }
}

impl RecipeList {
    fn view_entry(&self, (idx, recipe): (usize, &recipe::Recipe)) -> Html {
        html!{
            <tr>
                <td>
                    {"Recipe"}
                </td>
            </tr>
        }
    }
}

impl State {
    fn dummies(size: u32) -> Vec<recipe::Recipe> {
        let mut zero_vec: Vec<recipe::Recipe> = Vec::with_capacity(size as usize);
        for i in 0..size {
            zero_vec.push(recipe::Recipe::new());
        }
        return zero_vec;
    }
}