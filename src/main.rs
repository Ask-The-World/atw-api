mod conf_vars;
use mongodb::{Client};
use actix_web::{web, App, HttpServer, Responder};
mod db;


#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()>{

    struct AppState {
        client: Client
    }
    
    let client: Client = db::get_client().await?;

    println!("Successfully running... \nStop with CTRL + C...");
    
    async fn submit_question(web::Path(param): web::Path<(String, u32)>, data: web::Data<AppState>)-> impl Responder{
        let (question, time) = param;
        let x = db::ping_server(&data.client.clone()).await.unwrap();
        format!("Hello {:?}!, How are you, {:?}? - {:#?}", time, question, x)
    }

    HttpServer::new(move || 
        {App::new()
            .data(AppState{client: client.clone()})
            .service(web::scope("/api")
            .service(web::scope("/submit")
            .route("/question/{question}/{time}", web::get().to(submit_question))))})
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
