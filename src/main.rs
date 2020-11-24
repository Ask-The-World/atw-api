mod conf_vars;
use mongodb::{Client};
use tokio;
mod db;

#[tokio::main]
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
    return Ok(());
}
