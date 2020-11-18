use crate::{error::Error::*, handler::RecipeRequest, Recipe, Result};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};
use mongodb::results::{InsertOneResult, UpdateResult, DeleteResult};

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
        let doc: Document = doc! {
            RECIPE_NAME: entry.recipe_name.clone(),
        };

        let _result: InsertOneResult = self.get_recipe_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        //let _id = _result.inserted_id.to_string();
        let oid  =
            match _result.inserted_id {
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
        let doc: Document = doc! {
            RECIPE_NAME: entry.recipe_name.clone(),
        };

        let _result: UpdateResult =
            self.get_recipe_collection()
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

        let _result: DeleteResult =
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
        let recipe_uuid: &ObjectId = doc.get_object_id(RECIPE_UUID)?;
        let recipe_name: &str = doc.get_str(RECIPE_NAME)?;

        let recipe = Recipe {
            recipe_uuid: recipe_uuid.to_hex(),
            recipe_name: recipe_name.to_owned(),
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