mod conf_vars;
use actix_web::{web, App, HttpServer, Responder};
use mongodb::{Client, Collection, Database, bson::Document};
mod db;
use futures::stream::StreamExt;

#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()> {
    struct AppState {
        collection: Collection,
    }

    let client: Client = db::get_client().await?;
    let database: Database = client.database("atw");
    let collection: Collection = database.collection("questions");

    println!("Successfully running... \nStop with \"CTRL + C\"...");

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
