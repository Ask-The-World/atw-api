use mongodb::{Client, bson::doc, Collection, Cursor, Database};
use crate::conf_vars::{ConfVars, get_conf_vars};

pub async fn get_collection() -> mongodb::error::Result<Collection> {
    let config: ConfVars = get_conf_vars();
    let client = Client::with_uri_str(&format!("mongodb://{}:{}@{}:{}/", config.db_user, config.db_password, config.db_server, config.db_port)[..]).await?;
    let database: Database = client.database("atw");
    let collection: Collection = database.collection("questions");
    return Ok(collection)
}

pub async fn find_all(col: &Collection) ->  mongodb::error::Result<Cursor> {
    // Ping the server to see if you can connect to the cluster
    let cursor = col.find(doc! {}, None).await?;
    Ok(cursor)
}