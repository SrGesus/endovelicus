use std::collections::{BTreeMap, HashMap};

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
  config: Option<BTreeMap<String, String>>,
}

pub async fn create(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputCreate>,
) -> Result<String, StatusCode> {
  plugins
    .write()
    .unwrap()
    .insert(input.endpoint, input.plugin, input.config);
  Ok("Plugin created".to_owned())
}

#[derive(serde::Deserialize)]
pub struct InputRead {
  endpoint: Option<String>,
}

// FIXME: Find a way to avoid cloning for every read request
// even if PluginData is not big
pub async fn read(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputRead>,
) -> Result<Json<Plugins>, Error> {
  let plugins: std::sync::RwLockReadGuard<Plugins> = plugins.read().unwrap();
  if let Some(endpoint) = input.endpoint {
    if let Some(plugin) = plugins.0.get(&endpoint) {
      let mut map = HashMap::new();
      map.insert(endpoint, plugin.clone());
      Ok(Json(Plugins(map)))
    } else {
      Err(Error::NoSuchEntity("Plugin", "endpoint", endpoint))
    }
  } else {
    Ok(Json(plugins.clone()))
  }
}

pub async fn delete(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<InputRead>,
) -> Result<(), Error> {
  if let Some(endpoint) = input.endpoint {
    plugins
      .write()
      .unwrap()
      .0
      .remove(&endpoint)
      .ok_or_else(|| Error::NoSuchEntity("Plugin", "endpoint", endpoint))?;
    Ok(())
  } else {
    plugins.write().unwrap().0.clear();
    Ok(())
  }
}
