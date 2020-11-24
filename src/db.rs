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
    //println!("Connected successfully.");
    Ok(())
}

// Slower than normal function because client does not get passed and a new one has to be created
pub async fn ping_server_slow() ->  mongodb::error::Result<String> {

    let client = get_client().await?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    //println!("Connected successfully.");
    Ok("Connected successfully.".to_string())
}

// Only for debugging, does not return anything
pub async fn list_databases_slow() ->  mongodb::error::Result<()> {

    let client = get_client().await?;

    // List the names of the databases in that deployment.
    for _db_name in client.list_database_names(None, None).await? {
        //println!("{}", db_name);
    }

    Ok(())
}