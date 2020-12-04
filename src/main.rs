mod conf_vars;
use actix_web::{web, App, HttpServer, Responder};
use mongodb::{Collection, bson::{Document, oid}};
mod db;
use futures::stream::StreamExt;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

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
    #[derive(Deserialize, Serialize)]
    struct QuestionEntry {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<oid::ObjectId>,
        question: String,
        time: u32,
        yes: u32,
        no: u32,
        default_answer: bool,
        timestamp: u64,
    }

    let collection = db::get_collection().await.unwrap();

    println!("Successfully running... \nStarted at {}...\nStop with \"CTRL + C\"...", now);

    async fn submit_question(
        web::Path(param): web::Path<(String, u32)>,
        data: web::Data<AppState>,
    ) -> impl Responder {
        let (question, time) = param;
        let mut cursor = db::find_all(&data.collection.clone()).await.unwrap();
        
        let mut results: Vec<Document> = [].to_vec();
        // // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    results.push(document);
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        println!("{:?}", results.len());

        format!("Hello {:?}!, How are you, {:?}? - {:#?}", time, question, results)
    }

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                collection: collection.clone(),
            })
            .service(web::scope("/api").service(web::scope("/submit").route(
                "/question/{question}/{time}",
                web::get().to(submit_question),
            )))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
