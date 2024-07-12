use axum::extract::{Json, Path, State};
use axum::http::StatusCode;

use crate::AppState;

use super::{Plugin, Plugins};

pub async fn get(
  State(AppState(_, plugins)): State<AppState>,
  Path(endpoint): Path<String>,
  Json(input): Json<String>,
) -> Result<StatusCode, StatusCode> {
  match plugins.read().unwrap().0.get(&endpoint) {
    Some(plugin) => Ok(StatusCode::OK),
    None => Err(StatusCode::NOT_FOUND),
  }
}
