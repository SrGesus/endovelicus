use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use extism::Wasm;

use crate::api::{Error, Json};
use crate::AppState;

use super::Plugins;

#[derive(serde::Deserialize)]
pub struct InputCreate {
  endpoint: String,
  plugin: Wasm,
}

pub async fn create(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputCreate>,
) -> Result<String, StatusCode> {
  plugins
    .write()
    .unwrap()
    .insert(input.endpoint, input.plugin);
  Ok("Plugin created".to_owned())
}

#[derive(serde::Deserialize)]
pub struct InputRead {
  endpoint: String,
}

// 
pub async fn read(
  State(AppState(_, plugins)): State<AppState>,
  input: Option<Json<InputRead>>,
) -> Result<Json<Plugins>, Error> {
  let plugins: std::sync::RwLockReadGuard<Plugins> = plugins.read().unwrap();
  if let Some(Json(input)) = input {
    if let Some(plugin) = plugins.0.get(&input.endpoint) {
      let mut map = HashMap::new();
      map.insert(input.endpoint, plugin.clone());
      Ok(Json(Plugins(map)))
    } else {
      Err(Error::NoSuchEntity("Plugin", "endpoint", input.endpoint))
    }
  } else {
    Ok(Json(plugins.clone()))
  }
}

// pub async fn delete(
//   State(AppState(_, plugins)): State<AppState>,
//   Json(input): Json<InputRead>,
// ) -> String {
//   if let Some(endpoint) = input.endpoint {
//     plugins.write().unwrap().0.remove(&endpoint);
//     "Plugin deleted".to_owned()
//   } else {
//     plugins.write().unwrap().0.clear();
//     "All plugins deleted".to_owned()
//   }
// }
