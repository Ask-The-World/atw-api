mod conf_vars;
use mongodb::{Client, bson::doc};
use tokio;

#[tokio::main]
pub async fn main() -> mongodb::error::Result<()>{

    // Get configuration
    let config: conf_vars::ConfVars = conf_vars::get_conf_vars();
    println!("{}, {}, {}, {}, {},", config.min_time, config.max_time, config.default_time, config.max_question_length, config.default_delete_time);

    let client = Client::with_uri_str(&format!("mongodb://{}:{}@{}:{}/", config.db_user, config.db_password, config.db_server, config.db_port)[..]).await?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    return Ok(());
}
