use crate::{error::Error::*, handler::RecipeRequest, OvenFanValue, Result};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{options::ClientOptions, Client, Collection};
use std::str::FromStr;

use rcp_shared_rs_code::models::recipe::Recipe;
use rcp_shared_rs_code::models::step::Step;
use rcp_shared_rs_code::models::temperature::Temperature;
use rcp_shared_rs_code::models::{book_source::BookSource, ingredient::Ingredient};
use rcp_shared_rs_code::models::{haccp_value::HACCPValue, r#yield::Yield};
use rcp_shared_rs_code::models::{
    ingredient::{Amount, IngredientData},
    temperature_unit::TemperatureUnit,
};

const DB_NAME: &str = "rcp_db";
const RECIPE_COLL: &str = "recipe";

const RECIPE_UUID: &str = "_id";
const RECIPE_NAME: &str = "recipe_name";
const OVEN_TIME: &str = "oven_time";
const NOTES: &str = "notes";
const OVEN_FAN: &str = "oven_fan";
const OVEN_TEMP: &str = "oven_temp";
const SOURCE_BOOK: &str = "source_book";
const SOURCE_AUTHORS: &str = "source_authors";
const SOURCE_URL: &str = "source_url";
const INGREDIENTS: &str = "ingredients";
const STEPS: &str = "steps";
const YIELDS: &str = "yields";

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
            result.push(self.doc_to_recipe(doc?)?);
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
            self.doc_to_recipe(doc)
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
        println!("RecipeRequest to convert: {:#?}", recipe_request);
        let doc = bson::to_document(&recipe_request).unwrap();
        println!("Document: {:#?}", doc);
        doc

        // let oven_time: Bson = match recipe_request.oven_time {
        //     Some(t) => Bson::Double(t),
        //     None => Bson::Null,
        // };

        // let oven_fan = match &recipe_request.oven_fan {
        //     Some(ofv) => Bson::String(ofv.to_string()),
        //     None => Bson::Null,
        // };

        // let notes: Bson = match &recipe_request.notes {
        //     Some(s) => Bson::String(s.clone()),
        //     None => Bson::Null,
        // };

        // let oven_temp = match &recipe_request.oven_temp {
        //     Some(temp) => Bson::Document(self.temperature_to_doc(&temp)),
        //     None => Bson::Null,
        // };

        // let source_book = match &recipe_request.source_book {
        //     Some(bs) => Bson::Document(self.book_source_to_doc(&bs)),
        //     None => Bson::Null,
        // };

        // let source_authors = match &recipe_request.source_authors {
        //     Some(authors) => {
        //         let authors: Vec<Bson> = authors.iter().map(|s| Bson::String(s.clone())).collect();
        //         Bson::Array(authors)
        //     }
        //     None => Bson::Null,
        // };

        // let source_url = recipe_request
        //     .source_url
        //     .as_ref()
        //     .map_or(Bson::Null, |s| Bson::String(s.clone()));

        // let ingredients = match &recipe_request.ingredients {
        //     Some(i) => {
        //         let ingredients: Vec<Bson> = i
        //             .iter()
        //             .map(|i| Bson::Document(self.ingredient_to_doc(&i)))
        //             .collect();

        //         Bson::Array(ingredients)
        //     }
        //     None => Bson::Null,
        // };

        // let steps = match &recipe_request.steps {
        //     Some(v) => Bson::Array(
        //         v.iter()
        //             .map(|step| Bson::Document(self.step_to_doc(&step)))
        //             .collect(),
        //     ),
        //     None => Bson::Null,
        // };

        // let yields = match &recipe_request.yields {
        //     Some(v) => Bson::Array(
        //         v.iter()
        //             .map(|y| Bson::Document(self.yield_to_doc(&y)))
        //             .collect(),
        //     ),
        //     None => Bson::Null,
        // };

        // let doc: Document = doc! {
        //     RECIPE_NAME: recipe_request.recipe_name.clone(),
        //     OVEN_TIME: oven_time,
        //     NOTES: notes,
        //     OVEN_FAN: oven_fan,
        //     OVEN_TEMP: oven_temp,
        //     SOURCE_BOOK: source_book,
        //     SOURCE_AUTHORS: source_authors,
        //     SOURCE_URL: source_url,
        //     INGREDIENTS: ingredients,
        //     STEPS: steps,
        //     YIELDS: yields,
        // };

        // doc
    }

    fn doc_to_recipe(&self, doc: Document) -> Result<Recipe> {
        let recipe_uuid = doc.get_object_id(RECIPE_UUID)?.to_hex();
        let req: RecipeRequest = bson::from_document(doc)?;
        println!("Req: {:?}", req);
        let mut recipe = Recipe::from(req);
        recipe.recipe_uuid = recipe_uuid; 
        Ok(recipe)

        // let recipe_uuid: &ObjectId = doc.get_object_id(RECIPE_UUID)?;

        // let recipe_name: Option<String> = match doc.get(RECIPE_NAME).and_then(Bson::as_str) {
        //     Some(s) => Some(s.to_owned()),
        //     None => None,
        // };

        // let oven_time: Option<f64> = doc.get(OVEN_TIME).and_then(Bson::as_f64);

        // let oven_fan = match doc.get(OVEN_FAN).and_then(Bson::as_str) {
        //     Some(s) => OvenFanValue::from_string(s),
        //     None => None,
        // };

        // let notes: Option<String> = match doc.get(NOTES).and_then(Bson::as_str) {
        //     Some(s) => Some(s.to_owned()),
        //     None => None,
        // };

        // let oven_temp: Option<Temperature> = match doc.get(OVEN_TEMP).and_then(Bson::as_document) {
        //     Some(doc) => {
        //         let temp = self.doc_to_temperature(doc);

        //         if temp.is_ok() {
        //             // This code is ugly and could be improved
        //             Some(temp.unwrap())
        //         } else {
        //             None
        //         }
        //     }
        //     None => None,
        // };

        // let ingredients: Option<Vec<Ingredient>> =
        //     match doc.get(INGREDIENTS).and_then(Bson::as_array) {
        //         Some(v) => {
        //             let v: Vec<Option<&Document>> = v.iter().map(|b| b.as_document()).collect();

        //             let mut ingredients: Vec<Ingredient> = vec![];

        //             for doc in &v {
        //                 if let Some(doc) = doc {
        //                     let ingredient = self.doc_to_ingredient(doc).unwrap();
        //                     ingredients.push(ingredient);
        //                 }
        //             }
        //             Some(ingredients)
        //         }
        //         None => None,
        //     };

        // let source_url: Option<String> = match doc.get(SOURCE_URL).and_then(Bson::as_str) {
        //     Some(s) => Some(s.to_owned()),
        //     None => None,
        // };

        // let source_book: Option<BookSource> = match doc.get(SOURCE_BOOK).and_then(Bson::as_document)
        // {
        //     Some(d) => Some(self.doc_to_book_source(&d).unwrap()),
        //     None => None,
        // };

        // let source_authors: Option<Vec<String>> =
        //     match doc.get(SOURCE_AUTHORS).and_then(Bson::as_array) {
        //         Some(v) => Some(
        //             v.iter()
        //                 .map(|b| {
        //                     if let Some(s) = b.as_str() {
        //                         s.to_string()
        //                     } else {
        //                         "".to_string()
        //                     }
        //                 })
        //                 .collect(),
        //         ),
        //         None => None,
        //     };

        // let steps: Option<Vec<Step>> = doc.get(STEPS).and_then(Bson::as_array).map_or(None, |v| {
        //     let v: Vec<Option<&Document>> = v.iter().map(|b| b.as_document()).collect();

        //     let mut steps: Vec<Step> = vec![];

        //     for d in &v {
        //         if let Some(d) = d {
        //             steps.push(self.doc_to_step(d).unwrap());
        //         }
        //     }

        //     Some(steps)
        // });

        // let yields: Option<Vec<Yield>> =
        //     doc.get(YIELDS).and_then(Bson::as_array).map_or(None, |v| {
        //         let v: Vec<Option<&Document>> = v.iter().map(|b| b.as_document()).collect();

        //         let mut yields: Vec<Yield> = vec![];

        //         for d in &v {
        //             if let Some(d) = d {
        //                 yields.push(self.doc_to_yield(d).unwrap());
        //             }
        //         }

        //         Some(yields)
        //     });

        // let recipe = Recipe {
        //     recipe_uuid: recipe_uuid.to_hex(),
        //     recipe_name,
        //     oven_time,
        //     oven_fan,
        //     notes,
        //     oven_temp,
        //     ingredients,
        //     source_url,
        //     source_book,
        //     source_authors,
        //     steps,
        //     yields,
        // };
        // Ok(recipe)
    }

    // fn ingredient_to_doc(&self, i: &Ingredient) -> Document {
    //     let substitutions = match &i.substitutions {
    //         Some(v) => {
    //             let subs = v
    //                 .iter()
    //                 .map(|d| Bson::Document(self.ingredient_data_to_doc(&d)))
    //                 .collect();

    //             Bson::Array(subs)
    //         }
    //         None => Bson::Null,
    //     };

    //     doc! {
    //         "ingredient": self.ingredient_data_to_doc(&i.ingredient),
    //         "substitutions": substitutions,
    //     }
    // }

    // fn doc_to_ingredient(&self, doc: &Document) -> Result<Ingredient> {
    //     let ing_data_doc: &Document = doc.get("ingredient").and_then(Bson::as_document).unwrap();
    //     let ing_data = self.doc_to_ingredient_data(ing_data_doc)?;

    //     let substitutions: Option<Vec<IngredientData>> = doc
    //         .get("substitutions")
    //         .and_then(Bson::as_array)
    //         .map_or(None, |v| {
    //             let v: Vec<Option<&Document>> = v.iter().map(|b| b.as_document()).collect();

    //             let mut subs: Vec<IngredientData> = vec![];

    //             for d in &v {
    //                 if let Some(d) = d {
    //                     let ing_data = self.doc_to_ingredient_data(d).unwrap();
    //                     subs.push(ing_data);
    //                 }
    //             }

    //             Some(subs)
    //         });

    //     let ing = Ingredient {
    //         ingredient: ing_data,
    //         substitutions,
    //     };

    //     Ok(ing)
    // }

    // fn ingredient_data_to_doc(&self, ing_data: &IngredientData) -> Document {
    //     let doc = doc! {
    //         "amounts": match &ing_data.amounts {
    //             Some(amounts) => {
    //                 let amounts = amounts.iter().map(|a| {
    //                         Bson::Document(doc! {
    //                             "amount": Bson::Double(a.amount),
    //                             "unit": Bson::String(a.unit.clone()),
    //                         })
    //                     }).collect();

    //                 Bson::Array(amounts)
    //             },
    //             None => Bson::Null,
    //         },
    //         "processing": ing_data.processing.as_ref().map_or(Bson::Null, |v| {
    //             let v: Vec<Bson> = v.iter().map(|s| Bson::String(s.clone())).collect();
    //             Bson::Array(v)
    //         }),
    //         "notes": ing_data.notes.as_ref().map_or(Bson::Null, |s| Bson::String(s.clone())),
    //         "ingredient_name": ing_data.ingredient_name.as_ref().map_or(Bson::Null, |s| Bson::String(s.clone())),
    //     };

    //     doc
    // }

    // fn doc_to_ingredient_data(&self, doc: &Document) -> Result<IngredientData> {
    //     let amounts = doc
    //         .get("amounts")
    //         .and_then(Bson::as_array)
    //         .map_or(None, |v| {
    //             let v: Vec<Option<&Document>> = v.iter().map(|b| b.as_document()).collect();

    //             let mut amounts: Vec<Amount> = vec![];

    //             for d in &v {
    //                 if let Some(d) = d {
    //                     let a: f64 = d.get("amount").and_then(Bson::as_f64).unwrap();
    //                     let u: String = d.get("unit").and_then(Bson::as_str).unwrap().to_string();

    //                     let amount = Amount { amount: a, unit: u };
    //                     amounts.push(amount);
    //                 }
    //             }

    //             if amounts.is_empty() {
    //                 None
    //             } else {
    //                 Some(amounts)
    //             }
    //         });

    //     let processing = doc
    //         .get("processing")
    //         .and_then(Bson::as_array)
    //         .map_or(None, |v| {
    //             let v: Vec<Option<&str>> = v.iter().map(|b| b.as_str()).collect();

    //             let mut processings: Vec<String> = vec![];

    //             for s in &v {
    //                 if let Some(s) = s {
    //                     processings.push(s.to_string());
    //                 }
    //             }

    //             if processings.is_empty() {
    //                 None
    //             } else {
    //                 Some(processings)
    //             }
    //         });

    //     let notes = doc
    //         .get("notes")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()));

    //     let ingredient_name = doc
    //         .get("ingredient_name")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()));

    //     let ing_data = IngredientData {
    //         amounts,
    //         processing,
    //         notes,
    //         ingredient_name,
    //     };

    //     Ok(ing_data)
    // }

    // fn step_to_doc(&self, step: &Step) -> Document {
    //     let doc = doc! {
    //         "step": Bson::String(step.step.clone()),
    //         "haccp": step.haccp.as_ref().map_or(Bson::Null, |v| Bson::Document(self.haccp_value_to_doc(&v))),
    //         "notes": step.notes.as_ref().map_or(Bson::Null, |s| Bson::String(s.clone())),
    //     };

    //     doc
    // }

    // fn doc_to_step(&self, doc: &Document) -> Result<Step> {
    //     let step = doc
    //         .get("step")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()))
    //         .unwrap();

    //     let haccp = doc
    //         .get("haccp")
    //         .and_then(Bson::as_document)
    //         .map_or(None, |d| Some(self.doc_to_haccp_value(d).unwrap()));

    //     let notes = doc
    //         .get("notes")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()));

    //     let step = Step { step, haccp, notes };

    //     Ok(step)
    // }

    // fn haccp_value_to_doc(&self, hv: &HACCPValue) -> Document {
    //     doc! {
    //         "control_point": Bson::String(hv.control_point.clone()),
    //         "critical_control_point": Bson::String(hv.critical_control_point.clone()),
    //     }
    // }

    // fn doc_to_haccp_value(&self, doc: &Document) -> Result<HACCPValue> {
    //     let control_point = doc
    //         .get("control_point")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()))
    //         .unwrap();

    //     let critical_control_point = doc
    //         .get("critical_control_point")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()))
    //         .unwrap();

    //     let haccp_value = HACCPValue {
    //         control_point,
    //         critical_control_point,
    //     };

    //     Ok(haccp_value)
    // }

    // fn yield_to_doc(&self, y: &Yield) -> Document {
    //     doc! {
    //         "amount": Bson::Double(y.amount),
    //         "unit": Bson::String(y.unit.clone()),
    //     }
    // }

    // fn doc_to_yield(&self, doc: &Document) -> Result<Yield> {
    //     let amount = doc.get("amount").and_then(Bson::as_f64).unwrap();

    //     let unit = doc
    //         .get("unit")
    //         .and_then(Bson::as_str)
    //         .map_or(None, |s| Some(s.to_string()))
    //         .unwrap();

    //     let r#yield = Yield { amount, unit };

    //     Ok(r#yield)
    // }

    // fn temperature_to_doc(&self, t: &Temperature) -> Document {
    //     doc! {
    //         "amount": Bson::Double(t.amount),
    //         "unit": Bson::String(t.unit.to_string()),
    //     }
    // }

    // fn doc_to_temperature(&self, doc: &Document) -> Result<Temperature> {
    //     let amount = doc.get("amount").and_then(Bson::as_f64).expect("Error finding amount or converting to f64");

    //     let unit_str = doc.get("unit").and_then(Bson::as_str).expect("Error finding TemperatureUnit String");

    //     let unit = TemperatureUnit::from_str(unit_str).expect("Error converting TemperatureUnit to String");

    //     let t = Temperature { amount, unit };

    //     Ok(t)
    // }

    // fn book_source_to_doc(&self, bs: &BookSource) -> Document {
    //     let authors = bs.authors.iter().map(|s| Bson::String(s.clone())).collect();

    //     doc! {
    //         "authors": Bson::Array(authors),
    //         "title": Bson::String(bs.title.clone()),
    //         "isbn": bs.isbn.as_ref().map_or(Bson::Null, |s| Bson::String(s.clone())),
    //         "notes": bs.notes.as_ref().map_or(Bson::Null, |s| Bson::String(s.clone())),
    //     }
    // }

    // fn doc_to_book_source(&self, doc: &Document) -> Result<BookSource> {
    //     let authors = unimplemented!();
    //     let title = unimplemented!();
    //     let isbn = unimplemented!();
    //     let notes = unimplemented!();

    //     let bs = BookSource {
    //         authors,
    //         title,
    //         isbn,
    //         notes,
    //     };

    //     Ok(bs)
    // }
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
