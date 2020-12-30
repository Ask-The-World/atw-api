use serde::{Deserialize, Serialize};
use mongodb::{bson, Collection};
use crate::conf_vars;

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
    pub config: conf_vars::ConfVars,
}
