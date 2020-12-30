// imports
mod conf_vars;
mod handlers;
mod db;
mod errors;
mod structures;
use actix_web::{web, App, HttpServer};
use crate::errors::{UserError, UserErrorType};
use crate::handlers::*;
use conf_vars::ConfVars;
use crate::structures::*;








#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // initializing app
    

    let config: ConfVars = match conf_vars::get_conf_vars() {
        Ok(config) => config,
        _ => return Ok(()),
    };

    let (collection, connected) = db::get_collection().await.unwrap();

    if connected == false {
        return Ok(())
    }

    println!("Server successfully running...\nStop with \"CTRL + C\"...");

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
