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
    

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    #[get("/{id}")]
    pub async fn index(web::Path(id): web::Path<u32>)-> impl Responder{
        format!("Hello {}!, How are you?", id)
    }
    HttpServer::new(|| App::new().service(index))
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
