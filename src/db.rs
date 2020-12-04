// imports
use bson::oid::ObjectId;
use mongodb::{Client, bson::doc, bson, Collection, Database};
use crate::{QuestionEntry, QuestionResult, conf_vars::{ConfVars, get_conf_vars}};
use futures::stream::StreamExt;

// initializing connection with database
pub async fn get_collection() -> mongodb::error::Result<Collection> {
    let config: ConfVars = get_conf_vars();
    let client = Client::with_uri_str(&format!("mongodb://{}:{}@{}:{}/", config.db_user, config.db_password, config.db_server, config.db_port)[..]).await?;
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
pub async fn submit_question(col: &Collection, data: QuestionEntry) -> mongodb::error::Result<ObjectId> {
    let serialized_data = bson::to_bson(&data)?;
    let document = serialized_data.as_document().unwrap();
    let result = col.insert_one(document.to_owned(), None).await?;
    let id = result.inserted_id.as_object_id().unwrap().to_owned();
    Ok(id)
}