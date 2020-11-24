use mongodb::{Client, bson::doc};
use crate::conf_vars::{ConfVars, get_conf_vars};

pub async fn get_client() -> mongodb::error::Result<Client> {
    let config: ConfVars = get_conf_vars();
    let client = Client::with_uri_str(&format!("mongodb://{}:{}@{}:{}/", config.db_user, config.db_password, config.db_server, config.db_port)[..]).await?;
    return Ok(client)
}

pub async fn ping_server(client: &Client) ->  mongodb::error::Result<()> {
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    Ok(())
}