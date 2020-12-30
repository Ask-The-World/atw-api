// imports
mod conf_vars;
mod db;
mod errors;
mod handlers;
mod structures;
use crate::errors::{UserError, UserErrorType};
use crate::handlers::*;
use crate::structures::*;
use actix_web::{web, App, HttpServer};
use conf_vars::ConfVars;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // initializing app

    let config: ConfVars = match conf_vars::get_conf_vars() {
        Ok(config) => config,
        _ => {
            println!("Could not parse given environment variables, please check if all have the correct format");
            return Ok(());
        }
    };

    let (collection, connected) = db::get_collection().await.unwrap();

    if connected == false {
        return Ok(());
    }

    let server: String = format!("{}:{}", config.server_ip, config.server_port); 

    println!("Server successfully running on {}...\nStop with \"CTRL + C\"...", server);

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
                    .service(
                        web::scope("/get")
                            .route("/answer/{object_id}", web::get().to(get_answer))
                            .route("/question", web::get().to(get_question)),
                    )
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
    .bind(server)?
    .run()
    .await?;

    return Ok(());
}
