// imports
mod conf_vars;
use actix_web::{web, App, HttpServer, Responder};
use bson::oid::ObjectId;
use conf_vars::ConfVars;
use mongodb::{bson, Collection};
mod db;
use chrono::{Duration, Utc};
use rand::random;
use serde::{Deserialize, Serialize};

// question Formats
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubmitQuestion {
    question: String,
    time: i64,
    yes: i32,
    no: i32,
    default_answer: bool,
    expire_at: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionResult {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    question: String,
    time: u32,
    yes: u32,
    no: u32,
    default_answer: bool,
    expire_at: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetQuestion {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    question: String,
    time: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAnswer {
    question: String,
    time: bson::Bson,
    answer: bool,
}

#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()> {
    // initializing app
    struct AppState {
        collection: Collection,
        config: conf_vars::ConfVars,
    }

    let config: ConfVars = conf_vars::get_conf_vars();

    let collection = db::get_collection().await.unwrap();

    println!("Successfully running...\nStop with \"CTRL + C\"...");

    // route handlers

    // TODO: add error handling and returning status codes
    async fn submit_question(
        web::Path(param): web::Path<(String, u32)>,
        data: web::Data<AppState>,
    ) -> impl Responder {
        let (question, time) = param;

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
            .await
            .unwrap();

        web::Json(result)
    }

    // TODO: add error handling and returning status codes
    async fn list_all(data: web::Data<AppState>) -> impl Responder {
        let results = db::find_all(&data.collection.clone()).await.unwrap();
        web::Json(results)
    }

    // TODO: add error handling and returning status codes
    async fn get_question(data: web::Data<AppState>) -> impl Responder {
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
    async fn submit_answer(
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
    async fn get_answer(web::Path(object_id_string): web::Path<String>,
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

    // starting the server
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                collection: collection.clone(),
                config: config.clone(),
            })
            .service(
                web::scope("/api")
                    .route("/listall", web::get().to(list_all))
                    .service(web::scope("/get")
                            .route("/answer/{object_id}", web::get().to(get_answer))
                            .route("/question", web::get().to(get_question)))
                    .service(
                        web::scope("/submit")
                            .route(
                                "/question/{question}/{time}",
                                web::get().to(submit_question),
                            )
                            .route("/answer/{answer}/{object_id}", web::get().to(submit_answer)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
