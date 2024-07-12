use axum::{
  body::Body,
  extract::{rejection::JsonRejection, FromRequest},
  response::IntoResponse,
};

use super::error::Error;

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response<Body> {
    axum::response::Response::builder()
      .status(self.status_code())
      .header("Content-Type", "text/plain")
      .body(Body::from(self.to_string()))
      .unwrap()
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
    let Self(value) = self;
    axum::Json(value).into_response()
  }
}

impl From<JsonRejection> for Error {
  fn from(rejection: JsonRejection) -> Self {
    Error::ParsingFail(
      rejection.body_text().split_once(':').unwrap().1.to_owned(),
      rejection.status(),
    )
  }
}
