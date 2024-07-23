use axum::extract::{Json, Path, State};
use axum::http::Method;

use crate::error::Error;
use crate::plugins::SerPluginStore;

use crate::AppState;

pub async fn call(
  method: Method,
  State(AppState(_, plugins)): State<AppState>,
  Path((endpoint, function)): Path<(String, String)>,
  Json(input): Json<serde_json::Value>,
) -> Result<String, Error> {
  tracing::info!("Calling from method: {}", method);

  let mut plugin = plugins
    .read()
    .await
    .get_plugin(&endpoint)
    .ok_or(Error::NoSuchEntity("Plugin", "endpoint", endpoint))?
    .write_owned()
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

pub async fn get(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<Json<SerPluginStore>, Error> {
  let plugins = plugins.read().await;
  Ok(Json(if let Some(endpoint) = input.endpoint {
    plugins.search(&endpoint).await
  } else {
    plugins.to_serializable().await
  }))
}

pub async fn delete(
  State(AppState(_, plugins)): State<AppState>,
  Json(input): Json<OptionPlugin>,
) -> Result<(), Error> {
  let mut plugins = plugins.write().await;
  if let Some(endpoint) = input.endpoint {
    plugins
      .remove(&endpoint)
      .ok_or_else(|| Error::NoSuchEntity("Plugin", "endpoint", endpoint))?;
  } else {
    plugins.clear();
  }
  Ok(())
}
