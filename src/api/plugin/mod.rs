use axum::extract::{Json, Path, State};
use axum::http::Method;

use crate::error::Error;
use crate::plugins::{Config, PluginData, SerPluginStore};

use crate::AppState;

pub async fn call(
  method: Method,
  State(AppState(_, plugins)): State<AppState>,
  Path((endpoint, function)): Path<(String, String)>,
  Json(input): Json<serde_json::Value>,
) -> Result<String, Error> {
  tracing::info!("Calling from method: {}", method);
  let plugins = plugins.read().await;

  let mut plugin = plugins
    .0
    .get(&endpoint)
    .ok_or(Error::NoSuchEntity("Plugin", "endpoint", endpoint))?
    .write()
    .await;

  if !plugin.plugin_mut().function_exists(&function) {
    return Err(Error::NoSuchEntity("plugin function", "name", function));
  }

  plugin
    .plugin_mut()
    .call::<String, String>(function, input.to_string())
    .map_err(|err| Error::Plugin(err))
}

use std::collections::BTreeMap;

use axum::http::StatusCode;
use extism::Wasm;

#[derive(serde::Deserialize)]
pub struct OptionPlugin {
  endpoint: Option<String>,
  plugin: Option<Wasm>,
  config: Option<BTreeMap<String, String>>,
}

pub async fn get_config(plugin: &mut PluginData) -> Result<Option<Config>, Error> {
  plugin
    .plugin_mut()
    .call("config", "")
    .map_err(|err| Error::Plugin(err))
    .map(|cfg: Option<Config>| cfg.filter(|cfg| !cfg.0.is_empty()))
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
) -> Result<Json<SerPluginStore>, Error> {
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
    Ok(Json(SerPluginStore(map)))
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
