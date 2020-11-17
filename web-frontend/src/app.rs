use yew::prelude::*;
use yew_router::prelude::*;

use crate::recipe_list::RecipeList;
use crate::add_recipe::AddRecipeComp;

pub struct App {
    link: ComponentLink<Self>,
}

#[derive(Switch, Debug, Clone)]
pub enum RecipesRoute {

    #[to="/add"]
    AddRecipe,

    #[to=""]
    AllRecipes,


}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {

    // #[to = "/profile/{id}"]
    // Profile(u32),

    #[to = "/recipes{*:rest}"]
    Recipes(RecipesRoute),

    #[to = "/"]
    Home,
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
        html! {
            <>                
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {html!{<>
                            
                            { App::navigation_bar(&switch) }
                            <div class="ui center aligned container">
                                { App::content_view(&switch) }
                            </div>

                        </>}})
                    />
            </>
        }
    }
}

impl App {
    fn content_view(switch: &AppRoute) -> Html {
        match switch {
            AppRoute::Recipes(recipes_route) => {
                match recipes_route {
                    RecipesRoute::AddRecipe => html!{<>
                        { " ADD "}
                        // <AddRecipeComp />
                    </>},
                    RecipesRoute::AllRecipes => html!{<>
                        <RecipeList />
                    </>},
                }
            },
            AppRoute::Home => html!{<>
                { "Home" }
            </>},
        }
    }

    fn navbar_links(switch: &AppRoute) -> Html {
        let item: &str = "item";
        let active_item: &str = "active item";

        let home_link_classes = match switch {
            AppRoute::Home => active_item,
            _ => item,
        };

        let recipes_link_classes = match switch {
            AppRoute::Recipes(recipes_route) => {
                match recipes_route {
                    RecipesRoute::AllRecipes => active_item,
                    _ => item,
                }
            },
            _ => item,
        };

        let add_recipe_link_classes: &str = match switch {
            AppRoute::Recipes(recipes_route) => {
                match recipes_route {
                    RecipesRoute::AddRecipe => active_item,
                    _ => item,
                }
            },
            _ => item,
        };

        html!{<>
            <a class=home_link_classes href="/">{"Home"}</a>
            <a class=recipes_link_classes href="/recipes">{"Recipes"}</a>
            <a class=add_recipe_link_classes href="/recipes/add">{"Add Recipe"}</a>
        </>}

    }

    fn navigation_bar(switch: &AppRoute) -> Html {
        let navigation_bar = html! {<>
            <div class="ui tablet computer only padded grid">
                <div class="ui borderless fluid huge inverted menu">
                    <div class="ui container">
                        <a class="header item navbar-site-header" href="#root">{"Recipedia"}</a>
                        { App::navbar_links(&switch) }
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
                        { App::navbar_links(&switch) }
                    </div>
                </div>
            </div>
            </>};

        navigation_bar
    } 
}