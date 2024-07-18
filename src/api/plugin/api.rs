use std::collections::BTreeMap;

use axum::extract::State;
use axum::http::StatusCode;
use extism::Wasm;

use crate::api::Json;
use crate::error::Error;
use crate::AppState;

use super::SerPlugins;

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
    .await // Panics if Lock is poisoned
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

// FIXME find a way to return as Plugins instead of SerPlugins, but without cloning
pub async fn get(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<Json<SerPlugins>, Error> {
  if let Some(endpoint) = input.endpoint {
    // FIXME rewrite with collect instead
    let mut map = BTreeMap::new();
    if let Some(plugin) = plugins
      .read()
      .await // Panics if Lock is poisoned
      .0
      .get(&endpoint)
    {
      map.insert(endpoint, plugin.read().await.clone());
    }
    Ok(Json(SerPlugins(map)))
  } else {
    Ok(Json(plugins.read().await.to_serializable().await))
  }
}

pub async fn delete(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<(), Error> {
  if let Some(endpoint) = input.endpoint {
    plugins
      .write()
      .await
      .0
      .remove(&endpoint)
      .ok_or_else(|| Error::NoSuchEntity("Plugin", "endpoint", endpoint))?;
    Ok(())
  } else {
    plugins.write().await.0.clear();
    Ok(())
  }
}
