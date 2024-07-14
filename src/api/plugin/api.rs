use std::collections::{BTreeMap, HashMap};

use axum::extract::State;
use axum::http::StatusCode;
use extism::Wasm;

use crate::api::{Error, Json};
use crate::AppState;

use super::Plugins;

#[derive(serde::Deserialize)]
pub struct OptionPlugin {
  endpoint: Option<String>,
  plugin: Option<Wasm>,
  config: Option<BTreeMap<String, String>>,
}

pub async fn put(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<(StatusCode, String), Error> {
  let result = plugins
    .write()
    .unwrap() // Panics if Lock is poisoned
    .insert(
      input
        .endpoint
        .ok_or(Error::InvalidParameter("plugin is required for plugin."))?,
      input
        .plugin
        .ok_or(Error::InvalidParameter("plugin is required for plugin."))?,
      input.config,
    );
  match result {
    None => Ok((StatusCode::CREATED, "Plugin created.".to_owned())),
    Some(_) => Ok((StatusCode::OK, "Plugin replaced.".to_owned())),
  }
}

// FIXME: Find a way to avoid cloning for every read request
// even if PluginData is not big
pub async fn get(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<Json<Plugins>, Error> {
  if let Some(endpoint) = input.endpoint {
    let mut map = HashMap::new();
    if let Some(plugin) = plugins
      .read()
      .unwrap() // Panics if Lock is poisoned
      .0
      .get(&endpoint)
    {
      map.insert(endpoint, plugin.clone());
    }
    Ok(Json(Plugins(map)))
  } else {
    Ok(Json(plugins.read().unwrap().clone()))
  }
}

pub async fn delete(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
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
