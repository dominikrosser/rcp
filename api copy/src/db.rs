use crate::{error::Error::*, handler::RecipeRequest, Recipe, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "rcp_db";
const RECIPE_COLL: &str = "recipe";

const RECIPE_UUID: &str = "_id";
const RECIPE_NAME: &str = "recipe_name";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some(DB_NAME.to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_recipes(&self) -> Result<Vec<Recipe>> {
        let mut cursor = self
            .get_recipe_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Recipe> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_recipe(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_recipe(&self, entry: &RecipeRequest) -> Result<()> {
        let doc = doc! {
            RECIPE_NAME: entry.recipe_name.clone(),
        };

        self.get_recipe_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn edit_recipe(&self, id: &str, entry: &RecipeRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            RECIPE_UUID: oid,
        };
        let doc = doc! {
            RECIPE_NAME: entry.recipe_name.clone(),
        };

        self.get_recipe_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_recipe(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            RECIPE_UUID: oid,
        };

        self.get_recipe_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_recipe_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(RECIPE_COLL)
    }

    fn doc_to_recipe(&self, doc: &Document) -> Result<Recipe> {
        let recipe_uuid = doc.get_object_id(RECIPE_UUID)?;
        let recipe_name = doc.get_str(RECIPE_NAME)?;

        let recipe = Recipe {
            recipe_uuid: recipe_uuid.to_hex(),
            recipe_name: recipe_name.to_owned(),
        };
        Ok(recipe)
    }
}