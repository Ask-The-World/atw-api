// imports
use crate::{
    conf_vars::{get_conf_vars, ConfVars},
    QuestionResult, SubmitQuestion,
};
use crate::{UserError, UserErrorType};
use bson::{oid::ObjectId, Document};
use futures::stream::StreamExt;
use mongodb::{bson, bson::doc, results::UpdateResult, Client, Collection, Database};

// initializing connection with database
pub async fn get_collection() -> Result<(Collection, bool), UserError> {
    let config: ConfVars = match get_conf_vars() {
        Ok(config) => config,
        _ => {
            println!("Impossible to see this, if you are here something is seriously wrong with this code...");
            return Err(UserError {
                error_type: UserErrorType::InternalError,
                cause: None,
                message: Some(
                    "No clue what you did to get here... :) | pls contact me...".to_string(),
                ),
            });
        }
    };
    let client = Client::with_uri_str(
        &format!(
            "mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority",
            config.db_user, config.db_password, config.db_server, config.db_database
        )[..],
    )
    .await?;
    let database: Database = client.database("atw");
    let connected: bool;
    match database.run_command(doc! {"ping": 1}, None).await {
        Ok(_x) => {
            println!("Connected successfully to database ...");
            connected = true;
        }
        _ => {
            println!("Could not establish connection to database, please check credentials and try again");
            connected = false;
        }
    }
    let collection: Collection = database.collection("questions");
    return Ok((collection, connected));
}

// collecting all questions
pub async fn find_all(col: &Collection) -> Result<Vec<QuestionResult>, UserError> {
    let mut cursor = col.find(doc! {}, None).await?;

    let mut results: Vec<QuestionResult> = [].to_vec();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let doc_result: Result<QuestionResult, mongodb::bson::de::Error> =
                    bson::from_bson(bson::Bson::Document(document));
                match doc_result {
                    Ok(document) => results.push(document),
                    _ => {
                        return Err(UserError {
                            error_type: UserErrorType::SerializingError,
                            cause: None,
                            message: None,
                        })
                    }
                }
            }
            _ => {
                return Err(UserError {
                    error_type: UserErrorType::InternalError,
                    cause: None,
                    message: None,
                })
            }
        }
    }
    Ok(results)
}

// submiting a single question and returning ObjectId

pub async fn submit_question(
    col: &Collection,
    data: SubmitQuestion,
) -> Result<ObjectId, UserError> {
    let serialized_data = bson::to_bson(&data)?;
    let document_option = serialized_data.as_document();
    let document: &Document;
    match document_option {
        Some(doc) => document = doc,
        _ => {
            return Err(UserError {
                error_type: UserErrorType::SerializingError,
                cause: None,
                message: None,
            })
        }
    }
    let result = col.insert_one(document.to_owned(), None).await?;
    let id_option = result.inserted_id.as_object_id();
    match id_option {
        Some(id) => return Ok(id.to_owned()),
        _ => {
            return Err(UserError {
                error_type: UserErrorType::InternalError,
                cause: None,
                message: None,
            })
        }
    }
}

pub async fn get_random_question(col: &Collection) -> Result<QuestionResult, UserError> {
    let options = bson::from_document(doc! {"$sample": {"size": 1}});
    let mut cursor = col.aggregate(options, None).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let doc_result: Result<QuestionResult, mongodb::bson::de::Error> =
                    bson::from_bson(bson::Bson::Document(document));
                match doc_result {
                    Ok(document) => return Ok(document),
                    _ => {
                        return Err(UserError {
                            error_type: UserErrorType::SerializingError,
                            cause: None,
                            message: None,
                        })
                    }
                }
            }
            _ => {
                return Err(UserError {
                    error_type: UserErrorType::InternalError,
                    cause: None,
                    message: None,
                })
            }
        }
    }
    Err(UserError {
        error_type: UserErrorType::InternalError,
        cause: None,
        message: Some("Currently no questions in database".to_string()),
    })
}

pub async fn submit_answer(
    col: &Collection,
    object_id: bson::oid::ObjectId,
    answer: bool,
) -> mongodb::error::Result<UpdateResult> {
    match answer {
        true => Ok(col
            .update_one(doc! {"_id": object_id}, doc! {"$inc": {"yes": 1}}, None)
            .await?),
        false => Ok(col
            .update_one(doc! {"_id": object_id}, doc! {"$inc": {"no": 1}}, None)
            .await?),
    }
}

pub async fn get_answer(
    col: &Collection,
    object_id: bson::oid::ObjectId,
) -> Result<QuestionResult, UserError> {
    let result_option = col.find_one(doc! {"_id": object_id}, None).await?;
    let result: Document;
    match result_option {
        Some(document) => {
            result = document;
        }
        _ => {
            return Err(UserError {
                error_type: UserErrorType::InternalError,
                cause: None,
                message: Some("Could not find question".to_string()),
            })
        }
    }
    let answer_result: Result<QuestionResult, mongodb::bson::de::Error> =
        bson::from_bson(bson::Bson::Document(result));
    match answer_result {
        Ok(answer) => return Ok(answer),
        _ => {
            return Err(UserError {
                error_type: UserErrorType::SerializingError,
                cause: None,
                message: None,
            })
        }
    }
}
