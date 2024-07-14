use axum::extract::{Json, Path, State};
use axum::http::{Method, StatusCode};

use crate::AppState;

pub async fn call(
  method: Method,
  State(AppState(_, plugins)): State<AppState>,
  Path((endpoint, function)): Path<(String, String)>,
  Json(input): Json<serde_json::Value>,
) -> Result<String, StatusCode> {
  tracing::info!("Calling from method: {}", method);
  plugins
    .read()
    .unwrap()
    .get_mut(&endpoint)
    .ok_or(StatusCode::NOT_FOUND)?
    .plugin_mut()
    .call(function, input)
    .map_err(|err| {
      tracing::error!("{}", &err);
      if err.to_string().contains("not found") {
        StatusCode::NOT_FOUND
      } else {
        StatusCode::INTERNAL_SERVER_ERROR
      }
    })
}
