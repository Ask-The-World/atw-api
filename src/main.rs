mod conf_vars;
use actix_web::{web, App, HttpServer, Responder};
use mongodb::{Collection, bson,};
mod db;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use rand::random;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionEntry {
    question: String,
    time: i64,
    yes: i32,
    no: i32,
    default_answer: bool,
    timestamp: i128,
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
    timestamp: u64,
}

#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()> {
    struct AppState {
        collection: Collection,
    }

    let mut now: u64 = 0;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => now = time.as_secs(),
        Err(_) => {}
    }

    let collection = db::get_collection().await.unwrap();

    println!("Successfully running... \nStarted at {}...\nStop with \"CTRL + C\"...", now);

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
            timestamp: i128::from(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),

        };
        let result = db::submit_question(&data.collection.clone(), question_entry).await.unwrap();

        web::Json(result)
    }

    async fn list_all(data: web::Data<AppState>) -> impl Responder {
        let results = db::find_all(&data.collection.clone()).await.unwrap();
        web::Json(results)
    }

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                collection: collection.clone(),
            })
            .service(web::scope("/api")
            .route("/listall", web::get().to(list_all))
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
