use actix_web::{http::StatusCode, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("S3 error: {0}")]
    S3(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("SeaORM error: {0}")]
    SeaORM(#[from] sea_orm::error::DbErr),

    #[error("Not found")]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::S3(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Connection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SeaORM(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(ErrorResponse {
            status: self.status_code().as_u16(),
            message: self.to_string(),
        })
    }
}

pub trait NoneIs404<T> {
    fn none_is_404(self) -> Result<T, Error>
    where
        T: std::marker::Sized;
}

impl<T: Sized> NoneIs404<T> for Option<T> {
    fn none_is_404(self) -> Result<T, Error> {
        match self {
            Some(v) => Ok(v),
            None => Err(Error::NotFound),
        }
    }
}
