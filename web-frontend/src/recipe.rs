use yew::prelude::*;

pub struct Recipe {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Recipe {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Recipe {
            link,
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