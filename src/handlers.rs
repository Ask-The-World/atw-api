use rand::random;
use chrono::{Duration, Utc};
use actix_web::{web, Responder};
use crate::db;
use mongodb::bson;
use bson::oid::ObjectId;
use crate::structures::*;
use crate::{UserError, UserErrorType};

pub async fn submit_question(
    web::Path(param): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (question_in, time_in) = param;
    let question: String;
    let time: u32 = match time_in.parse::<u32>() {
        Ok(time) => time,
        _ => return Err(UserError{
            error_type: UserErrorType::BadRequest,
            cause: None,
            message: Some("Time was not specified as a u32 integer, please only provide numbers :)".to_string())
        })};
    if question_in.chars().count() as u32 <= data.config.max_question_length {
        question = question_in;
    }
    else {
        return Err(UserError{
            error_type: UserErrorType::BadRequest,
            cause: None,
            message: Some("Question was longer than maximum allowed characters - BadRequest :(".to_string())
        })
    }



    let question_entry = SubmitQuestion {
        question: question,
        time: i64::from(time),
        yes: 0,
        no: 0,
        default_answer: random(),
        expire_at: bson::Bson::DateTime(
            Utc::now()
                + Duration::seconds(
                    i64::from(time) + i64::from(data.config.default_delete_time),
                ),
        ),
    };
    let result = db::submit_question(&data.collection.clone(), question_entry)
        .await?;

    Ok(web::Json(result))
}

// TODO: add error handling and returning status codes
pub async fn list_all(data: web::Data<AppState>) -> impl Responder {
    let query = db::find_all(&data.collection.clone()).await;
    let results: Vec<QuestionResult>;
    match query {
        Ok(query) => {results = query;}
        _ => return Err(UserError{
            error_type: UserErrorType::InternalError,
            cause: None,
            message: Some("Database connection error".to_string())
        })
    }
    Ok(web::Json(results))
}

// TODO: add error handling and returning status codes
pub async fn get_question(data: web::Data<AppState>) -> impl Responder {
    let query = db::get_random_question(&data.collection.clone())
        .await
        .unwrap();
    let result: GetQuestion = GetQuestion {
        id: query.id.clone(),
        question: query.question,
        time: bson::Bson::DateTime(
            query.id.timestamp() + Duration::seconds(i64::from(query.time)),
        ),
    };
    web::Json(result)
}

// TODO: add error handling and returning status codes
pub async fn submit_answer(
    web::Path(param): web::Path<(bool, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (answer, object_id_string) = param;
    let object_id: ObjectId = bson::oid::ObjectId::with_string(&object_id_string[..]).unwrap();
    let result = db::submit_answer(&data.collection.clone(), object_id, answer)
        .await
        .unwrap();
    web::Json(result)
}

// TODO: add error handling and returning status codes
pub async fn get_answer(web::Path(object_id_string): web::Path<String>,
data: web::Data<AppState>,
) -> impl Responder{
    let object_id: ObjectId = bson::oid::ObjectId::with_string(&object_id_string[..]).unwrap();
    let result: QuestionResult = db::get_answer(&data.collection.clone(), object_id).await.unwrap();
    let mut answer: bool = result.default_answer;
    if result.yes > result.no {answer = false;}
    else if result.yes < result.no {answer = false;}
    let output: GetAnswer = GetAnswer{
        question: result.question,
        time: bson::Bson::DateTime(
            result.id.timestamp() + Duration::seconds(i64::from(result.time)),
        ),
        answer: answer,
    };
    web::Json(output)
}