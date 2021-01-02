use crate::db;
use crate::structures::*;
use crate::{UserError, UserErrorType};
use actix_web::{web, Responder};
use bson::oid::ObjectId;
use chrono::{Duration, Utc};
use mongodb::bson;
use rand::random;

pub async fn submit_question(
    web::Path(param): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (question_in, time_in) = param;
    let question: String;
    let time: u32 = match time_in.parse::<u32>() {
        Ok(time) => time,
        _ => {
            return Err(UserError {
                error_type: UserErrorType::BadRequest,
                cause: None,
                message: Some(
                    "Time was not specified as a u32 integer, please only provide numbers :)"
                        .to_string(),
                ),
            })
        }
    };
    if question_in.chars().count() as u32 <= data.config.max_question_length {
        question = question_in;
    } else {
        return Err(UserError {
            error_type: UserErrorType::BadRequest,
            cause: None,
            message: Some(
                "Question was longer than maximum allowed characters - BadRequest :(".to_string(),
            ),
        });
    }

    let question_entry = SubmitQuestion {
        question: question,
        time: i64::from(time),
        yes: 0,
        no: 0,
        default_answer: random(),
        expire_at: bson::Bson::DateTime(
            Utc::now()
                + Duration::seconds(i64::from(time) + i64::from(data.config.default_delete_time)),
        ),
    };
    let result = db::submit_question(&data.collection.clone(), question_entry).await?;

    Ok(web::Json(result))
}

pub async fn list_all(data: web::Data<AppState>) -> Result<impl Responder, UserError> {
    let results = db::find_all(&data.collection.clone()).await?;
    Ok(web::Json(results))
}

pub async fn get_question(data: web::Data<AppState>) -> Result<impl Responder, UserError> {
    let query: QuestionResult = db::get_random_question(&data.collection.clone()).await?;
    let result: GetQuestion = GetQuestion {
        id: query.id.clone(),
        question: query.question,
        time: bson::Bson::DateTime(query.id.timestamp() + Duration::seconds(i64::from(query.time))),
    };
    Ok(web::Json(result))
}

pub async fn submit_answer(
    web::Path(param): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<impl Responder, UserError> {
    let (answer_str, object_id_string) = param;
    let answer: bool =
        match answer_str.parse::<bool>() {
            Ok(answer) => answer,
            _ => return Err(UserError {
                error_type: UserErrorType::BadRequest,
                cause: None,
                message: Some(
                    "Answer was not specified as a boolean, please only provide true or false :)"
                        .to_string(),
                ),
            }),
        };
    let object_id: ObjectId = bson::oid::ObjectId::with_string(&object_id_string[..]).unwrap();
    let result = db::submit_answer(&data.collection.clone(), object_id, answer).await?;
    Ok(web::Json(result))
}

pub async fn get_answer(
    web::Path(object_id_string): web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, UserError> {
    let object_id: ObjectId = match bson::oid::ObjectId::with_string(&object_id_string[..]){
        Ok(object_id) => object_id,
        _ => return Err(UserError{
            error_type: UserErrorType::BadRequest,
            cause: None,
            message: Some("The provided object id could not be passed, please check if it is a valid bson object id :)".to_string())
        })
    };
    let result: QuestionResult = db::get_answer(&data.collection.clone(), object_id).await?;
    let mut answer: bool = result.default_answer;
    if result.yes > result.no {
        answer = false;
    } else if result.yes < result.no {
        answer = false;
    }
    let output: GetAnswer = GetAnswer {
        question: result.question,
        time: bson::Bson::DateTime(
            result.id.timestamp() + Duration::seconds(i64::from(result.time)),
        ),
        answer: answer,
    };
    Ok(web::Json(output))
}

pub async fn get_config(data: web::Data<AppState>) -> Result<impl Responder, UserError> {
    let config: ServerConfig = ServerConfig{
        min_time: data.config.min_time,
        max_time: data.config.max_time,
        default_time: data.config.default_time,
        max_question_length: data.config.max_question_length,
    };
    Ok(web::Json(config))
}