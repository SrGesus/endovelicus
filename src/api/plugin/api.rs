use std::collections::HashMap;

use axum::extract::{Json, State};
use axum::http::StatusCode;
use extism::Wasm;

use crate::AppState;

use super::{Plugin, Plugins};

#[derive(serde::Deserialize)]
pub struct InputCreate {
  endpoint: String,
  plugin: Plugin,
}

pub async fn create(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputCreate>,
) -> Result<String, StatusCode> {
  plugins.write().unwrap().0.insert(
    input.endpoint,
    Plugin {
      name: input.plugin.name.clone(),
      wasm: input.plugin.wasm.with_name(input.plugin.name),
    },
  );
  Ok("Plugin created".to_owned())
}

#[derive(serde::Deserialize)]
pub struct InputRead {
  endpoint: Option<String>,
}

pub async fn read(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputRead>,
) -> Json<Plugins> {
  Json(match input.endpoint {
    Some(endpoint) => Plugins(
      plugins
        .read()
        .unwrap()
        .0
        .get(&endpoint)
        .map(|p| {
          let mut map = HashMap::new();
          map.insert(endpoint, p.clone());
          map
        })
        .unwrap_or_default(),
    ),
    None => plugins.read().unwrap().clone(),
  })
}

pub async fn delete(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputRead>,
) -> String {
  if let Some(endpoint) = input.endpoint {
    plugins.write().unwrap().0.remove(&endpoint);
    "Plugin deleted".to_owned()
  } else {
    plugins.write().unwrap().0.clear();
    "All plugins deleted".to_owned()
  }
}
