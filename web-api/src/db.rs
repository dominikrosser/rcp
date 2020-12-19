use crate::{error::Error::*, handler::RecipeRequest, OvenFanValue, Result};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{options::ClientOptions, Client, Collection};

use rcp_shared_rs_code::models::recipe::Recipe;

const DB_NAME: &str = "rcp_db";
const RECIPE_COLL: &str = "recipe";

const RECIPE_UUID: &str = "_id";
const RECIPE_NAME: &str = "recipe_name";
const OVEN_TIME: &str = "oven_time";
const NOTES: &str = "notes";
const OVEN_FAN: &str = "oven_fan";

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

    pub async fn fetch_recipe(&self, id: &str) -> Result<Recipe> {
        let oid: ObjectId = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter: Document = doc! {
            RECIPE_UUID: oid,
        };
        let options = None; //todo
        let doc = self
            .get_recipe_collection()
            .find_one(filter, options)
            .await
            .map_err(MongoQueryError)?;

        if let Some(doc) = doc {
            self.doc_to_recipe(&doc)
        } else {
            Err(InvalidIDError(id.to_string()))
        }
    }

    pub async fn create_recipe(&self, entry: &RecipeRequest) -> Result<String> {
        let doc = self.doc_from_recipe_request(&entry);

        let _result: InsertOneResult = self
            .get_recipe_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        let oid = match _result.inserted_id {
            mongodb::bson::Bson::ObjectId(oid) => oid,
            _ => panic!("_id is not an ObjectId!"),
        };

        let recipe_uuid = oid.to_hex();

        Ok(recipe_uuid)
    }

    pub async fn edit_recipe(&self, id: &str, entry: &RecipeRequest) -> Result<()> {
        let oid: ObjectId = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query: Document = doc! {
            RECIPE_UUID: oid,
        };

        let doc = self.doc_from_recipe_request(&entry);

        let _result: UpdateResult = self
            .get_recipe_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn delete_recipe(&self, id: &str) -> Result<()> {
        let oid: ObjectId = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter: Document = doc! {
            RECIPE_UUID: oid,
        };

        let _result: DeleteResult = self
            .get_recipe_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_recipe_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(RECIPE_COLL)
    }

    fn doc_from_recipe_request(&self, recipe_request: &RecipeRequest) -> Document {
        let oven_time: Bson = match recipe_request.oven_time {
            Some(t) => Bson::Double(t),
            None => Bson::Null,
        };

        let oven_fan = match &recipe_request.oven_fan {
            Some(ofv) => Bson::Int32(ofv.to_database_code()),
            None => Bson::Null,
        };

        let notes: Bson = match &recipe_request.notes {
            Some(s) => Bson::String(s.clone()),
            None => Bson::Null,
        };

        let doc: Document = doc! {
            RECIPE_NAME: recipe_request.recipe_name.clone(),
            OVEN_TIME: oven_time,
            NOTES: notes,
            OVEN_FAN: oven_fan,
        };

        doc
    }

    fn doc_to_recipe(&self, doc: &Document) -> Result<Recipe> {
        let recipe_uuid: &ObjectId = doc.get_object_id(RECIPE_UUID)?;

        let recipe_name: Option<String> = match doc.get(RECIPE_NAME).and_then(Bson::as_str) {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        let oven_time: Option<f64> = doc.get(OVEN_TIME).and_then(Bson::as_f64);

        let oven_fan = match doc.get(OVEN_FAN).and_then(Bson::as_i32) {
            Some(i) => OvenFanValue::from_database_code(i),
            None => None,
        };

        let notes: Option<String> = match doc.get(NOTES).and_then(Bson::as_str) {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        let recipe = Recipe {
            recipe_uuid: recipe_uuid.to_hex(),
            recipe_name: recipe_name,
            oven_time: oven_time,
            oven_fan: oven_fan,
            notes: notes,
            oven_temp: None,
            ingredients: None,
            source_url: None,
            source_book: None,
            source_authors: None,
            steps: None,
            yields: None,
        };
        Ok(recipe)
    }
}

/* FILTER EXAMPLE */
// let filter = doc! { "author": "George Orwell" };
// let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
// let mut cursor = collection.find(filter, find_options).await?;

// // Iterate over the results of the cursor.
// while let Some(result) = cursor.next().await {
//     match result {
//         Ok(document) => {
//             if let Some(title) = document.get("title").and_then(Bson::as_str) {
//                 println!("title: {}", title);
//             }  else {
//                 println!("no title found");
//             }
//         }
//         Err(e) => return Err(e.into()),
//     }
// }
