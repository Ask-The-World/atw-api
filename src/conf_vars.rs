// Imports for working with environment variables and .env files
use dotenv::dotenv;
use std::env;

// Config Structure
pub struct ConfVars{
    pub min_time: u32,
    pub max_time: u32,
    pub default_time: u32,
    pub max_question_length: u32,
    pub default_delete_time: u32,
}

pub fn get_conf_vars() -> ConfVars {

    // Importing Environment variables from .env file
    dotenv().ok();

    // Assigning default values to the configuration variables
    let mut conf_vars: ConfVars = ConfVars{
        min_time: 30,
        max_time: 300,
        default_time: 180,
        max_question_length: 255,
        default_delete_time: 300};

    // Assigning the configuration values found in the environment variables
    for (key, value) in env::vars() {
        match &key[..] {
            "MIN_TIME" => {conf_vars.min_time = value.parse().unwrap();}
            "MAX_TIME" => {conf_vars.max_time = value.parse().unwrap();}
            "DEFAULT_TIME" => {conf_vars.default_time = value.parse().unwrap();}
            "MAX_QUESTION_LENGTH" => {conf_vars.max_question_length = value.parse().unwrap();}
            "DEFAULT_DELETE_TIME" => {conf_vars.default_delete_time = value.parse().unwrap();}
            _ => {}
        }
    }

    return conf_vars;
}