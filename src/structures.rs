use mongodb::{bson, Collection};
use serde::{Deserialize, Serialize};

// question formats
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubmitQuestion {
    pub question: String,
    pub time: i64,
    pub yes: i32,
    pub no: i32,
    pub default_answer: bool,
    pub expire_at: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionResult {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub question: String,
    pub time: u32,
    pub yes: u32,
    pub no: u32,
    pub default_answer: bool,
    pub expire_at: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetQuestion {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub question: String,
    pub time: bson::Bson,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetAnswer {
    pub question: String,
    pub time: bson::Bson,
    pub answer: bool,
}

pub struct AppState {
    pub collection: Collection,
    pub config: ConfVars,
}

// config structure
#[derive(Clone)]
pub struct ConfVars {
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerConfig {
    pub min_time: u32,
    pub max_time: u32,
    pub default_time: u32,
    pub max_question_length: u32,
}