use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;
use serde::{Serialize};

#[derive(Error, Debug, Serialize)]
pub enum Error {
    #[error("Logic error: {0}")]
    Logic(String)
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Logic => (StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}