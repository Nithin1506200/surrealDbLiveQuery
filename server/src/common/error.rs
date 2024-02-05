#![allow(dead_code)]
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Display, Error, Serialize)]
pub enum AppErrorType {
    Unauthorized,
    InternalServerError,
    BadRequest,
    NotFound,
    DbError,
}
impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            AppErrorType::DbError | AppErrorType::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppErrorType::BadRequest => StatusCode::BAD_REQUEST,
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(&self)
    }
}
impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        Self {
            cause: Some("database error".to_string()),
            message: Some(value.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}
