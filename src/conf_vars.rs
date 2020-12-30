// imports for working with environment variables and .env files
use dotenv::dotenv;
use std::env;

// config structure
#[derive(Clone)]
pub struct ConfVars{
    pub min_time: u32,
    pub max_time: u32,
    pub default_time: u32,
    pub max_question_length: u32,
    pub default_delete_time: u32,
    pub db_password: String,
    pub db_user: String,
    pub db_port: String,
    pub db_server: String,
    pub db_database: String,
    pub db_collection: String,
    pub server_ip: String,
    pub server_port: u32,
}

pub fn get_conf_vars() -> Result<ConfVars, std::num::ParseIntError> {

    // importing environment variables from .env file
    dotenv().ok();
    
    // assigning default values to the configuration variables
    let mut conf_vars: ConfVars = ConfVars{
        min_time: 30,
        max_time: 300,
        default_time: 180,
        max_question_length: 255,
        default_delete_time: 300,
        db_user: "".to_string(),
        db_password: "".to_string(),
        db_port: "27017".to_string(),
        db_server: "localhost".to_string(),
        db_database: "atw".to_string(),
        db_collection: "questions".to_string(),
        server_ip: "127.0.0.1".to_string(),
        server_port: 8080,
    };

    // assigning the configuration values found in the environment variables
    for (key, value) in env::vars() {
        match &key[..] {
            "MIN_TIME" => {conf_vars.min_time = value.parse()?;}
            "MAX_TIME" => {conf_vars.max_time = value.parse()?;}
            "DEFAULT_TIME" => {conf_vars.default_time = value.parse()?;}
            "MAX_QUESTION_LENGTH" => {conf_vars.max_question_length = value.parse()?;}
            "DEFAULT_DELETE_TIME" => {conf_vars.default_delete_time = value.parse()?;}
            "DB_USER" => {conf_vars.db_user = value}
            "DB_PASSWORD" => {conf_vars.db_password = value}
            "DB_PORT" => {conf_vars.db_port = value}
            "DB_SERVER" => {conf_vars.db_server = value}
            "DB_DATABASE" => {conf_vars.db_database = value}
            "DB_COLLECTION" => {conf_vars.db_collection = value}
            "SERVER_IP" => {conf_vars.server_ip = value}
            "SERVER_PORT" => {conf_vars.server_port = value.parse()?;}
            _ => {}
        }
    }

    return Ok(conf_vars);
}