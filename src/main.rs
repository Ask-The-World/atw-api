mod conf_vars;
use mongodb::{Client};
use actix_web::{get, web, App, HttpServer, Responder};
mod db;


#[actix_web::main]
pub async fn main() -> mongodb::error::Result<()>{

    struct AppState {
        // config: conf_vars::ConfVars, // currently not being used, maybe usefull for later
        client: Client
    }
    // Get configuration
    let config: conf_vars::ConfVars = conf_vars::get_conf_vars();
    println!("{}, {}, {}, {}, {},", config.min_time, config.max_time, config.default_time, config.max_question_length, config.default_delete_time);

    let client: Client = db::get_client().await?;

    let client_ref = client.clone();
    db::ping_server(&client_ref).await?;
    
    #[get("/{id}")]
    async fn index(web::Path(id): web::Path<u32>, data: web::Data<AppState>)-> impl Responder{
        db::list_databases_slow().await.unwrap();
        let x = db::ping_server(&data.client.clone()).await.unwrap();
        format!("Hello {}!, How are you? - {:#?}", id, x)
    }

    HttpServer::new(move || {App::new().data(AppState{/* config: config.clone(),  */client: client.clone()}).service(index)})
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    return Ok(());
}
