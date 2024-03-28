use std::{collections::HashMap, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::BadRequest(s) => s,
            Error::InternalServerError(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut result = HashMap::new();
        result.insert("error", format!("{}", self));
        HttpResponse::build(self.status_code()).json(result)
    }
}

impl From<horo_date_time::Error> for Error {
    fn from(value: horo_date_time::Error) -> Self {
        match value {
            horo_date_time::Error::InvalidDateTime(s) => Error::BadRequest(s),
            horo_date_time::Error::InvalidZone(s) => Error::BadRequest(s),
            horo_date_time::Error::Function(s) => Error::InternalServerError(s),
        }
    }
}

impl From<ziwei::Error> for Error {
    fn from(value: ziwei::Error) -> Self {
        match value {
            ziwei::Error::Function(s) => Error::InternalServerError(s),
            ziwei::Error::InvalidProcessDateTime(s) => Error::BadRequest(s),
        }
    }
}
