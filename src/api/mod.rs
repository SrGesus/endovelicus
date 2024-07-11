use axum::{body::Body, http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use serde::Serializer;

use thiserror::Error;

pub mod account;
pub mod currency;
pub mod plugin;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Internal server error. Check the logs for more information.")]
  UnknownDbError(DbErr),
  #[error("Could not find {0} with {1} = {2}")]
  NoSuchEntity(&'static str, &'static str, String),
  #[error("There already exists a {0} with {1} = {2}")]
  DuplicateEntity(&'static str, &'static str, String),
  #[error("{0}")]
  InvalidInput(String),
}

impl Error {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Error::UnknownDbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::NoSuchEntity(_, _, _) => StatusCode::BAD_REQUEST,
      Error::DuplicateEntity(_, _, _) => StatusCode::CONFLICT,
      Error::InvalidInput(_) => StatusCode::BAD_REQUEST,
    }
  }
}

impl Into<Response> for Error {
  fn into(self) -> Response {
    Response {
      status: self.status_code(),
      message: Some(self.to_string()),
      data: serde_json::Value::Null,
    }
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response<Body> {
    Into::<Response>::into(self).into_response()
  }
}

impl From<DbErr> for Error {
  fn from(err: DbErr) -> Self {
    Error::UnknownDbError(err)
  }
}

#[derive(serde::Serialize)]
pub struct Response {
  #[serde(serialize_with = "serialize_status")]
  status: StatusCode,
  message: Option<String>,
  data: serde_json::Value,
}

fn code_to_status(value: &StatusCode) -> String {
  if value.as_u16() < 400 {
    "success".to_owned()
  } else if value.as_u16() < 500 {
    "fail".to_owned()
  } else {
    "error".to_owned()
  }
}

fn serialize_status<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&code_to_status(value))
}

// implement from arbitrary json for ApiResponse
impl From<serde_json::Value> for Response {
  fn from(value: serde_json::Value) -> Self {
    Self {
      status: StatusCode::OK,
      message: None,
      data: value,
    }
  }
}

impl IntoResponse for Response {
  fn into_response(self) -> axum::response::Response<Body> {
    let body = serde_json::to_string(&self).unwrap();
    axum::response::Response::builder()
      .status(self.status)
      .header("Content-Type", "application/json")
      .body(Body::from(body))
      .unwrap()
  }
}
