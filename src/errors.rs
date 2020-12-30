use actix_web::{error, http::StatusCode, HttpResponse};
use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum UserErrorType {
    InternalError,
    BadRequest,
    SerializingError,
    DbError
}

#[derive(Debug)]
pub struct UserError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: UserErrorType,
}

impl UserError {
    pub fn message(&self) -> String {
        match &*self {
            UserError {
                message: Some(message),
                ..
            } => message.clone(),
            UserError {
                message: None,
                error_type: UserErrorType::BadRequest,
                ..
            } => "Your request was not complete or false :(".to_string(),
            UserError {
                message: None,
                error_type: UserErrorType::InternalError,
                ..
            } => "Something wrent wrong in the server :(".to_string(),
            UserError {
                message: None,
                error_type: UserErrorType::SerializingError,
                ..
            } => "Could not serialize a data structure".to_string(),
            UserError {
                message: None,
                error_type: UserErrorType::DbError,
                ..
            } => "Database error, connection might not work".to_string(),
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            UserErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserErrorType::BadRequest => StatusCode::BAD_REQUEST,
            UserErrorType::SerializingError => StatusCode::BAD_REQUEST,
            UserErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

}

impl From<mongodb::bson::ser::Error> for UserError {
    fn from(error: mongodb::bson::ser::Error) -> UserError {
        UserError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: UserErrorType::SerializingError
        }
    }
}
impl From<mongodb::bson::de::Error> for UserError {
    fn from(error: mongodb::bson::de::Error) -> UserError {
        UserError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: UserErrorType::SerializingError
        }
    }
}

impl From<mongodb::error::Error> for UserError {
    fn from(error: mongodb::error::Error) -> UserError {
        UserError {
            message: None, 
            cause: Some(error.to_string()),
            error_type: UserErrorType::DbError
        }
    }
}