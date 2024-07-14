use thiserror::Error;

use axum::http::StatusCode;
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Internal server error. Check the logs for more information.")]
  Unknown(anyhow::Error),
  #[error("Plugin error: {0}")]
  Plugin(anyhow::Error),
  #[error("Could not find {0} with {1}: {2}")]
  NoSuchEntity(&'static str, &'static str, String),
  #[error("There already exists a {0} with {1}: {2}")]
  DuplicateEntity(&'static str, &'static str, String),
  #[error("Invalid parameter: {0}")]
  InvalidParameter(&'static str),
  #[error("Parsing error:{0}")]
  ParsingFail(String, StatusCode),
}

impl Error {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Error::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::Plugin(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::NoSuchEntity(_, _, _) => StatusCode::BAD_REQUEST,
      Error::DuplicateEntity(_, _, _) => StatusCode::CONFLICT,
      Error::InvalidParameter(_) => StatusCode::BAD_REQUEST,
      Error::ParsingFail(_, status) => *status,
    }
  }
}

impl From<DbErr> for Error {
  fn from(err: DbErr) -> Self {
    Error::Unknown(err.into())
  }
}
