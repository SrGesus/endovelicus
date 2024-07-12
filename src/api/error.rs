use axum::http::StatusCode;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Internal server error. Check the logs for more information.")]
  UnknownDbError(DbErr),
  #[error("Could not find {0} with {1} = {2}")]
  NoSuchEntity(&'static str, &'static str, String),
  #[error("There already exists a {0} with {1} = {2}")]
  DuplicateEntity(&'static str, &'static str, String),
  #[error("{0}")]
  InvalidParameter(&'static str),
  #[error("Parsing error:{0}")]
  ParsingError(String, StatusCode),
}

impl Error {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Error::UnknownDbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::NoSuchEntity(_, _, _) => StatusCode::BAD_REQUEST,
      Error::DuplicateEntity(_, _, _) => StatusCode::CONFLICT,
      Error::InvalidParameter(_) => StatusCode::BAD_REQUEST,
      Error::ParsingError(_, status) => *status,
    }
  }
}

impl From<DbErr> for Error {
  fn from(err: DbErr) -> Self {
    Error::UnknownDbError(err)
  }
}
