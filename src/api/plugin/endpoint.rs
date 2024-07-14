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
  let plugins = plugins.read().await;
  let plugin = plugins.0
    .get(&endpoint).ok_or(StatusCode::NOT_FOUND)?;
  let result = plugin.write().await.plugin_mut().call::<String, String>(function, input.to_string());
  result.map_err(|err| {
    if err.to_string().contains("not found") {
      StatusCode::NOT_FOUND
    } else {
      StatusCode::INTERNAL_SERVER_ERROR
    }
  })
}
