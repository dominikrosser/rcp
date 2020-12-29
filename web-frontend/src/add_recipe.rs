use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use yew::agent::{Dispatched, Dispatcher};
use yew::callback::Callback;
use yew::events::ChangeData;
use yew::events::MouseEvent;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew_router::{route::Route, service::RouteService, Switch};

use rcp_shared_rs_code::models::ingredient::Ingredient;
use rcp_shared_rs_code::models::oven_fan_value::OvenFanValue;
use rcp_shared_rs_code::models::r#yield::Yield;
use rcp_shared_rs_code::models::recipe_request::RecipeRequest;
use rcp_shared_rs_code::models::step::Step;
use rcp_shared_rs_code::models::temperature::Temperature;
use rcp_shared_rs_code::models::temperature_unit::TemperatureUnit;

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
    OnAddRecipe,
    OnReceivePostResponse(Result<CreateRecipeResponse, anyhow::Error>),
    OnRecipeNameInputChanged(String),
    OnRecipeOvenTimeInputChanged(String),
    OnRecipeNotesInputChanged(String),
    OnRecipeOvenFanSelectChanged(String),
    OnRecipeOvenTempAmountInputChanged(String), //TODO
    OnRecipeOvenTempUnitInputChanged(String),   //TODO
    // SOURCE BOOK
    // SOURCE AUTHORS
    OnRecipeSourceUrlInputChanged(String), //TODO
    // INGREDIENTS
    OnAddIngredient,
    OnIngredientNameInputChanged(usize, String),
    // (ingredient index, amount index, amount.amount value)
    OnIngredientAmountInputChanged(usize, usize, String),
    // (ingredient index, amount index, amount.unit value)
    OnIngredientAmountUnitInputChanged(usize, usize, String),
    // (ingredient index, processing index, new value)
    OnIngredientProcessingInputChanged(usize, usize, String),
    // (ingredient index)
    OnIngredientAddProcessing(usize),
    // (ingredient index)
    OnIngredientAddNotes(usize),
    // (ingredient index, new value)
    OnIngredientNotesInputChanged(usize, String),
    // (ingredient index)
    OnRemoveIngredient(usize),
    // STEPS
    OnAddStep,
    // (step index, new step string)
    OnStepInputChanged(usize, String),
    // (step index)
    OnRemoveStep(usize),
    // (step index)
    OnStepAddNotes(usize),
    // (step index, new value)
    OnStepNotesInputChanged(usize, String),
    // YIELDS
    OnAddYield,
    // (yield index)
    OnRemoveYield(usize),
    // (yield index, new_amount_str)
    OnYieldAmountInputChanged(usize, String),
    // (yield index, new_unit_str)
    OnYieldUnitInputChanged(usize, String),
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
            Msg::OnAddRecipe => {
                let task: FetchTask = self.build_fetch_recipe_task();

                // 4. store the task so it isn't canceled immediately
                self.state.post_recipes_task = Some(task);

                true
            }
            Msg::OnReceivePostResponse(data) => {
                self.state.recipe_data.recipe_name = None;
                self.state.recipe_data.oven_time = None;
                self.state.recipe_data.oven_fan = None;
                self.state.post_recipes_task = None;

                match data {
                    Ok(recipe_response) => {
                        let new_route = format!("/recipes/{}", recipe_response.recipe_uuid);
                        ConsoleService::log(&format!("Reroute to: {}", new_route));
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
            Msg::OnRecipeNameInputChanged(recipe_name) => {
                self.state.recipe_data.recipe_name = Some(recipe_name);
                true
            }
            Msg::OnRecipeOvenTimeInputChanged(oven_time) => {
                let oven_time: f64 = oven_time.parse::<f64>().unwrap_or(0.0f64);
                self.state.recipe_data.oven_time = Some(oven_time);
                true
            }
            Msg::OnRecipeNotesInputChanged(notes) => {
                self.state.recipe_data.notes = Some(notes);
                true
            }
            Msg::OnRecipeOvenFanSelectChanged(oven_fan) => {
                self.state.recipe_data.oven_fan = OvenFanValue::from_str(&oven_fan).ok();
                true
            }
            Msg::OnRecipeOvenTempAmountInputChanged(amount_str) => {
                let amount: f64 = amount_str.parse().unwrap_or(0.0f64);
                self.state.recipe_data.oven_temp = self
                    .state
                    .recipe_data
                    .oven_temp
                    .as_ref()
                    .map_or(None, |temp| {
                        Some(Temperature {
                            amount,
                            unit: temp.unit.clone(),
                        })
                    });
                true
            }
            Msg::OnRecipeOvenTempUnitInputChanged(unit_str) => {
                let unit = TemperatureUnit::from_str(&unit_str).unwrap();
                self.state.recipe_data.oven_temp = self
                    .state
                    .recipe_data
                    .oven_temp
                    .as_ref()
                    .map_or(None, |temp| {
                        Some(Temperature {
                            amount: temp.amount,
                            unit,
                        })
                    });
                true
            }
            Msg::OnRecipeSourceUrlInputChanged(source_url_str) => {
                self.state.recipe_data.source_url = Some(source_url_str);
                true
            }
            Msg::OnAddYield => {
                if let Some(yields) = self.state.recipe_data.yields.as_mut() {
                    yields.push(Yield::new());
                } else {
                    self.state.recipe_data.yields = Some(vec![Yield::new()]);
                }
                true
            }
            Msg::OnAddStep => {
                if let Some(steps) = self.state.recipe_data.steps.as_mut() {
                    steps.push(Default::default());
                } else {
                    self.state.recipe_data.steps = Some(vec![Default::default()]);
                }
                true
            }
            Msg::OnAddIngredient => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    ingredients.push(Ingredient::new());
                } else {
                    self.state.recipe_data.ingredients = Some(vec![Ingredient::new()]);
                }
                true
            }
            Msg::OnIngredientNameInputChanged(idx, ing_name_str) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if idx < ingredients.len() {
                        ingredients[idx].ingredient.ingredient_name = ing_name_str;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientAmountInputChanged(ing_idx, amount_idx, amount_str) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        if amount_idx < ing.amounts.len() {
                            let value: f64 = amount_str.parse().unwrap_or(0.0f64);
                            ing.amounts[amount_idx].amount = value;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientAmountUnitInputChanged(ing_idx, amount_idx, unit_str) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        if amount_idx < ing.amounts.len() {
                            ing.amounts[amount_idx].unit = unit_str;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientProcessingInputChanged(ing_idx, processing_idx, new_str) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        if processing_idx < ing.processing.len() {
                            ing.processing[processing_idx] = new_str;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientAddProcessing(ing_idx) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        ing.processing.push(String::new());
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientAddNotes(ing_idx) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        if ing.notes.is_none() {
                            ing.notes = Some(String::new());
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnIngredientNotesInputChanged(ing_idx, new_notes_str) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        let ing = &mut ingredients[ing_idx].ingredient;
                        ing.notes = Some(new_notes_str);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnRemoveIngredient(ing_idx) => {
                if let Some(ingredients) = self.state.recipe_data.ingredients.as_mut() {
                    if ing_idx < ingredients.len() {
                        ingredients.remove(ing_idx);
                        if ingredients.is_empty() {
                            self.state.recipe_data.ingredients = None;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnStepInputChanged(step_idx, new_step_str) => {
                if let Some(steps) = self.state.recipe_data.steps.as_mut() {
                    if step_idx < steps.len() {
                        steps[step_idx].step = new_step_str;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnRemoveStep(step_idx) => {
                if let Some(steps) = self.state.recipe_data.steps.as_mut() {
                    if step_idx < steps.len() {
                        steps.remove(step_idx);
                        if steps.is_empty() {
                            self.state.recipe_data.steps = None;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnStepAddNotes(step_idx) => {
                if let Some(steps) = self.state.recipe_data.steps.as_mut() {
                    if step_idx < steps.len() {
                        let step = &mut steps[step_idx];
                        if step.notes.is_none() {
                            step.notes = Some(String::new());
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnStepNotesInputChanged(step_idx, new_notes_str) => {
                if let Some(steps) = self.state.recipe_data.steps.as_mut() {
                    if step_idx < steps.len() {
                        let step = &mut steps[step_idx];
                        if step.notes.is_some() {
                            step.notes = Some(new_notes_str);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnRemoveYield(yield_idx) => {
                if let Some(yields) = self.state.recipe_data.yields.as_mut() {
                    if yield_idx < yields.len() {
                        yields.remove(yield_idx);
                        if yields.is_empty() {
                            self.state.recipe_data.yields = None;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnYieldAmountInputChanged(yield_idx, amount_str) => {
                if let Some(yields) = self.state.recipe_data.yields.as_mut() {
                    if yield_idx < yields.len() {
                        let amount: f64 = amount_str.parse::<f64>().unwrap_or(0.0f64);
                        yields[yield_idx].amount = amount;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::OnYieldUnitInputChanged(yield_idx, unit_str) => {
                if let Some(yields) = self.state.recipe_data.yields.as_mut() {
                    if yield_idx < yields.len() {
                        yields[yield_idx].unit = unit_str;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn view(&self) -> Html {
        if self.state.post_recipes_task.is_some() {
            html! {<div class="ui medium text loader active">{ "Uploading..."}</div>}
        } else {
            let oninput: Callback<InputData> = self
                .link
                .callback(|e: InputData| Msg::OnRecipeNameInputChanged(e.value));

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
                    { self.view_oven_temp_input() }
                    { self.view_oven_fan_input() }
                    { self.view_notes_input() }
                    { self.view_steps_input() }
                    { self.view_yields_input() }
                    { self.view_ingredients_input() }
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
                Msg::OnReceivePostResponse(data)
            },
        );

        // 3. pass the request and callback to the fetch service
        let task =
            FetchService::fetch(post_request, callback).expect("failed to start post request");

        task
    }

    fn view_submit_recipe_button(&self) -> Html {
        html! {
            <button class="ui button" type="submit" onclick=self.link.callback(|_| Msg::OnAddRecipe)>
                { "Submit" }
            </button>
        }
    }

    fn view_oven_temp_input(&self) -> Html {
        match &self.state.recipe_data.oven_temp {
            None => html! {},
            Some(temp) => {
                let on_oven_temp_amount_input = self
                    .link
                    .callback(|e: InputData| Msg::OnRecipeOvenTempAmountInputChanged(e.value));

                html! {

                    <div class="field">
                        <label for="oven_temp_amount_input">{"Oven Temp."}</label>
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
                <label for="oven_fan_select">{"Oven Fan"}</label>
                <select
                    name="oven_fan",
                    id="oven_fan_select",
                    value={if let Some(of) = &self.state.recipe_data.oven_fan { OvenFanValue::to_string(of) } else { "".to_string() }},
                    onchange=self.link.callback(|e: ChangeData| Msg::OnRecipeOvenFanSelectChanged(match e {
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
                <label for="notes_input">{"Notes"}</label>
                <textarea
                    rows=4,
                    type="text",
                    id="notes_input",
                    value=match &self.state.recipe_data.notes {
                        None => "",
                        Some(s) => s,
                    },
                    oninput=self.link.callback(|e: InputData| Msg::OnRecipeNotesInputChanged(e.value))
                    />
            </div>
        }
    }

    fn view_oven_time_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="oven_time_input">{"Oven Time"}</label>
                <input
                    type="number"
                    id="oven_time_input"
                    value=match &self.state.recipe_data.oven_time {
                        None => "".to_string(),
                        Some(t) => t.to_string(),
                    },
                    oninput=self.link.callback(|e: InputData| Msg::OnRecipeOvenTimeInputChanged(e.value))
                    />
            </div>
        }
    }

    fn view_recipe_name_input(&self) -> Html {
        html! {
            <div class="field">
                <label for="recipe_name_input">{"Title"}</label>
                <input
                    type="text",
                    id="recipe_name_input"
                    value=match &self.state.recipe_data.recipe_name {
                        None => "",
                        Some(name) => name,
                    },
                    oninput=self.link.callback(|e: InputData| Msg::OnRecipeNameInputChanged(e.value))
                    />
            </div>
        }
    }

    fn view_step_input(&self, (idx, s): (usize, &Step)) -> Html {
        html! {<>
            <h3>{ format!("Step {}", idx + 1) }</h3>

            // Step String
            <div class="field">
                <label>
                    {"Instructions"}
                    <input
                        type="text",
                        value=&s.step,
                        oninput=self.link.callback(move |e: InputData| Msg::OnStepInputChanged(idx, e.value))
                        />
                </label>
            </div>

            // Step notes
            {
                if let Some(notes) = s.notes.as_ref() {
                    html! {
                        <div class="field">
                            <label>
                                {"Notes"}
                                <input
                                    type="text",
                                    value=notes,
                                    oninput=self.link.callback(move |e: InputData| Msg::OnStepNotesInputChanged(idx, e.value))
                                    />
                            </label>
                        </div>
                    }
                } else {
                    html! {
                        { self.view_add_btn("Add notes", move |_| Msg::OnStepAddNotes(idx)) }
                    }
                }
            }

            // TODO haccp_value
        </>}
    }

    fn view_steps_input(&self) -> Html {
        match &self.state.recipe_data.steps {
            Some(steps) => {
                let steps_html = html! {
                    for steps
                        .iter()
                        .enumerate()
                        .map(|(pos, entry)| self.view_step_input((pos, entry)))
                };

                html! {<>
                    { steps_html }
                    <br/>
                    { self.view_add_btn("", |_| Msg::OnAddStep) }
                </>}
            }
            None => {
                html! {
                    { self.view_add_btn("Add Steps", |_| Msg::OnAddStep) }
                }
            }
        }
    }

    fn view_ingredient_input(&self, (idx, i): (usize, &Ingredient)) -> Html {
        let ing = &i.ingredient;
        let subs = &i.substitutions;

        html! {<>
            // ingredient_name
            <div class="field">
                <label>
                    {"Name"}
                    <input
                        placeholder="e.g. apple(s)",
                        type="text",
                        value=&ing.ingredient_name,
                        oninput=self.link.callback(move |e: InputData| Msg::OnIngredientNameInputChanged(idx, e.value))
                    />
                </label>
            </div>

            // amount(s) TODO think about how to handle multiple amounts/yields
            {
                html! {
                    for ing.amounts
                        .iter()
                        .enumerate()
                        .map(|(pos, entry)| {
                            html! {<>
                                <div class="field">
                                    // amount
                                    <label>
                                        {"Amount"}
                                        <input
                                            type="number",
                                            value=&entry.amount,
                                            oninput=self.link.callback(move |e: InputData|
                                                Msg::OnIngredientAmountInputChanged(idx, pos, e.value)
                                            )
                                            />
                                    </label>
                                </div>
                                // unit
                                <div class="field">
                                    <label>
                                        {"Unit"}
                                        <input
                                            type="text"
                                            value=&entry.unit,
                                            oninput=self.link.callback(move |e: InputData|
                                                Msg::OnIngredientAmountUnitInputChanged(idx, pos, e.value)
                                            )
                                            />
                                    </label>
                                </div>
                            </>}
                            })
                }
            }

            // processing
            {
                html! {
                    for ing.processing.iter().enumerate().map(|(pos, entry)| {
                        html! {<>
                            <input
                                type="text",
                                value=&entry,
                                oninput=self.link.callback(move |e: InputData|
                                    Msg::OnIngredientProcessingInputChanged(idx, pos, e.value)
                                ) />
                        </>}
                    })
                }
            }
            { self.view_add_btn("Add Processing", move |_| Msg::OnIngredientAddProcessing(idx)) }

            // notes
            {
                if let Some(notes) = ing.notes.as_ref() {
                    html!{
                        <textarea
                            rows=2,
                            type="text",
                            value=notes,
                            oninput=self.link.callback(move |e: InputData| Msg::OnIngredientNotesInputChanged(idx, e.value))
                            />
                    }
                } else {
                    { self.view_add_btn("Add notes", move |_| Msg::OnIngredientAddNotes(idx)) }
                }
            }

            // Column for removing ingredients btn
                { self.view_remove_btn("Remove Ingredient", move |_| Msg::OnRemoveIngredient(idx)) }

        </>}
    }

    fn view_ingredients_input(&self) -> Html {
        match &self.state.recipe_data.ingredients {
            Some(ingredients) => {
                let ingredients_html = html! {
                    for ingredients
                        .iter()
                        .enumerate()
                        .map(|(pos, entry)| self.view_ingredient_input((pos, entry)))
                };

                let ingredients_list_html = html! {<>
                    <h3>{"Ingredients"}</h3>
                    { self.view_add_btn("Add Ingredient", |_| Msg::OnAddIngredient) }
                    { ingredients_html }
                </>};

                ingredients_list_html
            }
            None => {
                html! {
                    { self.view_add_btn("Add Ingredients", |_| Msg::OnAddIngredient) }
                }
            }
        }
    }

    fn view_add_btn<P: 'static>(&self, text: &str, cb: P) -> Html
    where
        P: Fn(MouseEvent) -> Msg,
    {
        html! {
            <div class="ui teal labeled icon button" onclick=self.link.callback(cb)>
                {text}
                <i class="add icon"></i>
            </div>
        }
    }

    fn view_remove_btn<P: 'static>(&self, text: &str, cb: P) -> Html
    where
        P: Fn(MouseEvent) -> Msg,
    {
        html! {
            <div class="ui labeled icon button" onclick=self.link.callback(cb)>
                {text}
                <i class="remove icon"></i>
            </div>
        }
    }

    fn view_yield_input(&self, (idx, y): (usize, &Yield)) -> Html {
        html! {
            <>
                // Amount
                <div class="field">
                    <input
                        type="number",
                        value=&y.amount,
                        oninput=self.link.callback(move |e: InputData| Msg::OnYieldAmountInputChanged(idx, e.value))
                        />
                </div>

                // Unit
                <div class="field">
                    <input
                        type="text",
                        value=&y.unit,
                        oninput=self.link.callback(move |e: InputData| Msg::OnYieldUnitInputChanged(idx, e.value))
                        />
                </div>
            </>
        }
    }

    fn view_yields_input(&self) -> Html {
        match &self.state.recipe_data.yields {
            Some(yields) => {
                let yields_html = html! {
                    for yields
                        .iter()
                        .enumerate()
                        .map(|(pos, entry)| self.view_yield_input((pos, entry)))
                };

                yields_html
            }
            None => self.view_add_btn("Add Yields", |_| Msg::OnAddYield),
        }
    }
}
