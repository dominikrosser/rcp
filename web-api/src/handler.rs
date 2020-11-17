use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeRequest {
    pub recipe_name: String,
}

pub async fn recipes_list_handler(db: DB) -> WebResult<impl Reply> {
    let recipes = db.fetch_recipes().await.map_err(|e| reject::custom(e))?;
    Ok(json(&recipes))
}

pub async fn recipe_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let recipe = db.fetch_recipe(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&recipe))
}

pub async fn create_recipe_handler(body: RecipeRequest, db: DB) -> WebResult<impl Reply> {
    db.create_recipe(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_recipe_handler(id: String, body: RecipeRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_recipe(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_recipe_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_recipe(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}