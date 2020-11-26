use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{route::Route, service::RouteService, Switch};

use crate::add_recipe::AddRecipeComp;
use crate::recipe::RecipeComp;
use crate::recipe_list::RecipeList;
use crate::reroute_agent::RerouteAgent;

pub type RouterStateType = ();
pub type RouteType = Route<RouterStateType>;
pub type RouteServiceType = RouteService<RouterStateType>;

pub struct App {
    link: ComponentLink<Self>,
    route_service: RouteServiceType,
    _reroute_agent_bridge: Box<dyn Bridge<RerouteAgent>>,
    route: RouteType,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum RecipesRoute {
    #[to = "/add"]
    AddRecipe,

    #[to = "/{id}"]
    ViewRecipe { id: String },

    #[to = ""]
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

pub enum Msg {
    ChangeRoute(String),
    RouteChange(RouteType),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let raw_route = route_service.get_route();
        let route = Route::from(raw_route);

        let callback = link.callback(|r| Msg::RouteChange(r));
        route_service.register_callback(callback);
        let rab = RerouteAgent::bridge(link.callback(|r: String| Msg::ChangeRoute(r)));

        App {
            link,
            route_service,
            _reroute_agent_bridge: rab,
            route,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(route_string) => {
                self.route_service.set_route(&route_string, ());
                let raw_route = self.route_service.get_route();
                let route = Route::from(raw_route);
                self.route = route;
                // TODO

                true
            }
            Msg::RouteChange(r) => {
                let raw_route = self.route_service.get_route();
                self.route = Route::from(raw_route);
                true
            }
        }
    }

    fn view(&self) -> Html {
        let switch = AppRoute::switch(self.route.clone());

        if let Some(switch) = switch {
            html! {
                <>
                    { self.navigation_bar(&switch) }
                    <div class="ui left aligned container">
                        { App::content_view(&switch) }
                    </div>
                </>
            }
        } else {
            html! { {"404"} }
        }
    }
}

impl App {
    fn change_route(&self, route: String) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = route.clone();
            Msg::ChangeRoute(route)
        })
    }

    fn content_view(switch: &AppRoute) -> Html {
        match switch {
            AppRoute::Recipes(recipes_route) => match recipes_route {
                RecipesRoute::AddRecipe => html! {<>
                    <AddRecipeComp />
                </>},
                RecipesRoute::AllRecipes => html! {<>
                    <RecipeList />
                </>},
                RecipesRoute::ViewRecipe { id } => html! {<>
                    <RecipeComp recipe_uuid=id />
                </>},
            },
            AppRoute::Home => html! {<>
                { "Home" }
            </>},
        }
    }

    fn navbar_links(&self, switch: &AppRoute) -> Html {
        let item: &str = "item";
        let active_item: &str = "active item";

        let home_link_classes = match switch {
            AppRoute::Home => active_item,
            _ => item,
        };

        let recipes_link_classes = match switch {
            AppRoute::Recipes(recipes_route) => match recipes_route {
                RecipesRoute::AllRecipes => active_item,
                _ => item,
            },
            _ => item,
        };

        let add_recipe_link_classes: &str = match switch {
            AppRoute::Recipes(recipes_route) => match recipes_route {
                RecipesRoute::AddRecipe => active_item,
                _ => item,
            },
            _ => item,
        };

        html! {<>
            <a
                class=home_link_classes,
                onclick=&self.change_route("/".to_string())>
                {"Home"}
            </a>

            <a
                class=recipes_link_classes,
                onclick=&self.change_route("/recipes/".to_string())>
                {"Recipes"}
            </a>

            <a
                class=add_recipe_link_classes,
                onclick=&self.change_route("/recipes/add".to_string())>
                {"Add Recipe"}
            </a>

        </>}
    }

    fn navigation_bar(&self, switch: &AppRoute) -> Html {
        let navigation_bar = html! {<>
        <div class="ui tablet computer only padded grid">
            <div class="ui borderless fluid huge inverted menu">
                <div class="ui container">
                    <a class="header item navbar-site-header" href="#root">{"Recipedia"}</a>
                    { self.navbar_links(&switch) }
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
                    { self.navbar_links(&switch) }
                </div>
            </div>
        </div>
        </>};

        navigation_bar
    }
}

