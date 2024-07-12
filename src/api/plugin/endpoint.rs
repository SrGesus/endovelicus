use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use extism::Manifest;

use crate::AppState;

use super::{Plugin, Plugins};

pub async fn get(
  State(AppState(_, plugins)): State<AppState>,
  Path(endpoint): Path<String>,
  Json(input): Json<serde_json::Value>,
) -> Result<String, StatusCode> {
  match plugins.write().unwrap().get_plugin(&endpoint) {
    Some(plugin) => {
      plugin.call("get", input).map_err(|err| {
        tracing::error!("{}", &err);
        if err.to_string().contains("not found") {
          StatusCode::NOT_FOUND
        } else {
          StatusCode::INTERNAL_SERVER_ERROR
        }
      })
    }
    None => Err(StatusCode::NOT_FOUND),
  }
}
