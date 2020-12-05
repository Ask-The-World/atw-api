// imports
use bson::oid::ObjectId;
use mongodb::{Client, Collection, Database, results::UpdateResult, bson::doc, bson};
use crate::{SubmitQuestion, QuestionResult, conf_vars::{ConfVars, get_conf_vars}};
use futures::stream::StreamExt;

// initializing connection with database
pub async fn get_collection() -> mongodb::error::Result<Collection> {
    let config: ConfVars = get_conf_vars();
    let client = Client::with_uri_str(&format!("mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority", config.db_user, config.db_password, config.db_server, config.db_database)[..]).await?;
    let database: Database = client.database("atw");
    let collection: Collection = database.collection("questions");
    return Ok(collection)
}

// collecting all questions
// TODO: add error handling and status codes
pub async fn find_all(col: &Collection) ->  mongodb::error::Result<Vec<QuestionResult>> {
    // Ping the server to see if you can connect to the cluster
    let mut cursor = col.find(doc! {}, None).await?;
        
    let mut results: Vec<QuestionResult> = [].to_vec();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let doc: QuestionResult= bson::from_bson(bson::Bson::Document(document)).unwrap();
                results.push(doc);
            }
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(results)
}

// submiting a single question and returning ObjectId
// TODO: add error handling and returning status codes
pub async fn submit_question(col: &Collection, data: SubmitQuestion) -> mongodb::error::Result<ObjectId> {
    let serialized_data = bson::to_bson(&data)?;
    let document = serialized_data.as_document().unwrap();
    let result = col.insert_one(document.to_owned(), None).await?;
    let id = result.inserted_id.as_object_id().unwrap().to_owned();
    Ok(id)
}

pub async fn get_random_question(col: &Collection) -> mongodb::error::Result<QuestionResult> {
    let options = bson::from_document(doc!{"$sample": {"size": 1}});
    let mut cursor = col.aggregate(options, None).await?;
    let mut question: Option<QuestionResult> = None;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let doc: QuestionResult = bson::from_bson(bson::Bson::Document(document)).unwrap();
                question = Some(doc);
            }
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(question.unwrap())
}

pub async fn submit_answer(col: &Collection, object_id: bson::oid::ObjectId, answer: bool) -> mongodb::error::Result<UpdateResult> {
    match answer {
        true  => {Ok(col.update_one(doc!{"_id": object_id}, doc!{"$inc": {"yes": 1}}, None).await?)},
        false => {Ok(col.update_one(doc!{"_id": object_id}, doc!{"$inc": {"no": 1}}, None).await?)}
    }
}

pub async fn get_answer(col: &Collection, object_id: bson::oid::ObjectId) -> mongodb::error::Result<QuestionResult> {
    let result = col.find_one(doc!{"_id": object_id}, None).await?;
    let answer: QuestionResult = bson::from_bson(bson::Bson::Document(result.unwrap())).unwrap();
    Ok(answer)
}