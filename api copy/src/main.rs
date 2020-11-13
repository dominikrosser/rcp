use chrono::prelude::*;
use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};
use warp::http::Method;
use warp::http::header::HeaderName;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub recipe_uuid: String,
    pub recipe_name: String,
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
        .or(recipe
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_recipe_handler))
        .or(recipe
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_recipe_handler))
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