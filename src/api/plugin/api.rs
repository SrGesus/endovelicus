use axum::extract::{Json, State};
use axum::http::StatusCode;
use extism::Wasm;

use crate::AppState;

use super::Plugins;

#[derive(serde::Deserialize)]
pub enum PluginInput {
  Path(String),
  Url(String),
}

#[derive(serde::Deserialize)]
pub struct InputCreate {
  endpoint: String,
  name: String,
  plugin: PluginInput,
}

pub async fn create(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputCreate>,
) -> Result<String, StatusCode> {
  let wasm = match input.plugin {
    PluginInput::Path(path) => Wasm::file(&path),
    PluginInput::Url(url) => Wasm::url(&url),
  }
  .with_name(input.name.clone());
  plugins.write().unwrap().insert(input.endpoint, input.name, wasm);
  Ok("Plugin created".to_owned())
}

pub async fn read(State(AppState(_, plugins)): State<AppState>) -> Json<Plugins> {
  Json(plugins.read().unwrap().clone())
}
