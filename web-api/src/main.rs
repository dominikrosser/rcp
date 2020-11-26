use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, http::Method, Rejection};
use warp::http::header::HeaderName;

use rcp_shared_rs_code::models::oven_fan_value::OvenFanValue;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub recipe_uuid: String,

    /* The name of this recipe. */
    pub recipe_name: String,

    /* How long the dish should spend in the oven.
     * This is an overall value, which refers to the recipe as a whole. If multiple oven times are used, they should be specified in the recipe. */
    pub oven_time: Option<f64>,

     /* This is a field that will appear in several locations. The recipe itself may have notes, each ingredient may have notes, and each step may have notes. */
    pub notes: Option<String>,

    /* Setting to be used with convection oven. Possible values are “Off”, “Low” and “High”. If not specified, it is assumed to be “Off”.
     * If specified, all software should display and print this value. If not specified, it is up to the software whether or not it is displayed and/or printed,
     * but it should be consistent. */
    pub oven_fan: Option<OvenFanValue>,
}

impl Recipe {
    pub fn new() -> Self {
        Self {
            recipe_uuid: "".to_string(),
            recipe_name: "".to_string(),
            oven_time: None,
            notes: None,
            oven_fan: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let recipe = warp::path("recipe");

    let recipe_routes = recipe
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_recipe_handler)

        // PUT "recipe/{id}"
        .or(recipe
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_recipe_handler))

        // DELETE "recipe/{id}"
        .or(recipe
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_recipe_handler))
        
        // GET "recipe/{id}"
        .or(recipe
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::recipe_handler))

        // GET "/recipe"
        .or(recipe
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::recipes_list_handler));

    let routes = recipe_routes
        .with(warp::cors()
            .allow_any_origin()
            .allow_header(HeaderName::from_lowercase(b"content-type").unwrap())
            .allow_method(Method::POST))
        .recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
