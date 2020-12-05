// imports
mod conf_vars;
use actix_web::{web, App, HttpServer, Responder};
use conf_vars::ConfVars;
use mongodb::{Collection, bson,};
mod db;
use serde::{Serialize, Deserialize};
use rand::random;
use chrono::{Duration, Utc};

// question Formats
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionEntry {
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

        let question_entry = QuestionEntry {
            question: question,
            time: i64::from(time),
            yes: 0,
            no: 0,
            default_answer: random(),
            expire_at: bson::Bson::DateTime(Utc::now() + Duration::seconds(i64::from(time) + i64::from(data.config.default_delete_time)))
        };
        let result = db::submit_question(&data.collection.clone(), question_entry).await.unwrap();

        web::Json(result)
    }

    // TODO: add error handling and returning status codes
    async fn list_all(data: web::Data<AppState>) -> impl Responder {
        let results = db::find_all(&data.collection.clone()).await.unwrap();
        web::Json(results)
    }

    // TODO: add error handling and returning status codes
    async fn get_question(data: web::Data<AppState>) -> impl Responder {
        let result = db::get_random_question(&data.collection.clone()).await.unwrap();
        web::Json(result)
    }

    // starting the server
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                collection: collection.clone(),
                config: config.clone(),
            })
            .service(web::scope("/api")
            .route("/listall", web::get().to(list_all))
            .service(web::scope("/get").route(
                "/question",
                web::get().to(get_question)))
            .service(web::scope("/submit").route(
                "/question/{question}/{time}",
                web::get().to(submit_question),
            )))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
