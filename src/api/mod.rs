use axum::{body::Body, http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use serde::Serializer;

pub mod account;
pub mod currency;
pub mod plugin;

#[derive(serde::Serialize)]
pub enum Status {
  Success,
  Fail,
  Error,
}

#[derive(serde::Serialize)]
pub struct Response {
  #[serde(serialize_with = "code_to_status")]
  status_code: u16,
  status: Status,
  message: Option<String>,
  data: serde_json::Value,
}

fn code_to_status<S>(value: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  // For demonstration, convert the string to uppercase before serialization
  // let modified_value = value.to_uppercase();
  // serializer.serialize_str(&modified_value)
  if *value < 400 {
    serializer.serialize_str("success")
  } else if *value < 500 {
    serializer.serialize_str("fail")
  } else {
    serializer.serialize_str("error")
  }
}

// implement from arbitrary for ApiResponse
impl From<serde_json::Value> for Response {
  fn from(value: serde_json::Value) -> Self {
    Self {
      status_code: StatusCode::OK.as_u16(),
      status: Status::Success,
      message: None,
      data: value,
    }
  }
}

impl From<DbErr> for Response {
  fn from(err: DbErr) -> Self {
    let status_code = match err {
      sea_orm::DbErr::RecordNotFound(_) => StatusCode::NOT_FOUND,
      sea_orm::DbErr::RecordNotInserted => StatusCode::BAD_REQUEST,
      sea_orm::DbErr::RecordNotUpdated => StatusCode::BAD_REQUEST,
      sea_orm::DbErr::TryIntoErr { .. } => StatusCode::BAD_REQUEST,
      sea_orm::DbErr::ConvertFromU64(_) => StatusCode::BAD_REQUEST,
      sea_orm::DbErr::Type(_) => StatusCode::BAD_REQUEST,
      sea_orm::DbErr::Json(_) => StatusCode::BAD_REQUEST,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
    .as_u16();
    Self {
      status_code,
      status: if status_code < 500 {
        Status::Fail
      } else {
        Status::Error
      },
      message: Some(format!("Database error: {}", err)),
      data: serde_json::Value::Null,
    }
  }
}

impl IntoResponse for Response {
  fn into_response(self) -> axum::response::Response<Body> {
    let body = serde_json::to_string(&self).unwrap();
    axum::response::Response::builder()
      .status(StatusCode::from_u16(self.status_code).unwrap())
      .header("Content-Type", "application/json")
      .body(Body::from(body))
      .unwrap()
  }
}
