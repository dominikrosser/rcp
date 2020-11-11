use yew::prelude::*;

use super::recipe::Recipe;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
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

        let navigation_bar = html! {<>
            <div class="ui tablet computer only padded grid">
                <div class="ui borderless fluid huge inverted menu">
                    <div class="ui container">
                        <a class="header item navbar-site-header" href="#root">{"Recipedia"}</a>
                        <a class="active item" href="#root">{"Home"}</a>
                    </div>
                </div>
            </div>
            <div class="ui mobile only padded grid">
                <div class="ui borderless fluid huge inverted menu">
                    <a class="header item" href="#root">{"Recipedia"}</a>
                    <div class="right menu">
                        <div class="item">
                            <button class="ui icon toggle basic inverted button">
                                <i class="content icon"></i>
                            </button>
                        </div>
                    </div>
                    <div class="ui vertical borderless fluid inverted menu">
                        <a class="active item" href="#root">{"Home"}</a>
                    </div>
                </div>
            </div>
            </>};

        html! {
            <>
                { navigation_bar }

                <div class="ui center aligned container">
                    <Recipe/>
                   { "Hello World!!!" } 
                </div>
            </>
        }
    }
}