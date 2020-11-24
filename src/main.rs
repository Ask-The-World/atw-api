mod conf_vars;
use mongodb::{Client};
use actix_web::{get, web, App, HttpServer, Responder};
mod db;

#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()>{

    // Get configuration
    let config: conf_vars::ConfVars = conf_vars::get_conf_vars();
    println!("{}, {}, {}, {}, {},", config.min_time, config.max_time, config.default_time, config.max_question_length, config.default_delete_time);

    let client: Client = db::get_client().await?;

    db::ping_server(&client).await?;
    
    #[get("/{id}")]
    async fn index(web::Path(id): web::Path<u32>)-> impl Responder{
        db::list_databases_slow().await.unwrap();
        let x = db::ping_server_slow().await.unwrap();
        format!("Hello {}!, How are you? - {}", id, x)
    }

    HttpServer::new(|| App::new().service(index))
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
