use axum::{
  body::Body,
  extract::{rejection::JsonRejection, FromRequest},
  http::StatusCode,
  response::IntoResponse,
};
use serde::Serializer;

use super::error::Error;

#[derive(serde::Serialize)]
struct Response {
  #[serde(serialize_with = "serialize_status")]
  status: StatusCode,
  data: serde_json::Value,
  message: Option<String>,
}

fn code_to_status(value: &StatusCode) -> &'static str {
  if value.as_u16() < 400 {
    "success"
  } else if value.as_u16() < 500 {
    "fail"
  } else {
    "error"
  }
}

fn serialize_status<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(code_to_status(value))
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

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response<Body> {
    Response {
      status: self.status_code(),
      message: Some(self.to_string()),
      data: serde_json::Value::Null,
    }
    .into_response()
  }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
  T: serde::Serialize,
{
  fn into_response(self) -> axum::response::Response {
    Response {
      status: StatusCode::OK,
      message: None,
      data: serde_json::json!(self.0),
    }
    .into_response()
  }
}

impl From<JsonRejection> for Error {
  fn from(rejection: JsonRejection) -> Self {
    Error::ParsingError(
      rejection
        .body_text()
        .splitn(2, ':')
        .nth(1)
        .unwrap()
        .to_owned(),
      rejection.status(),
    )
  }
}
