use axum::extract::{Json, Path, State};
use axum::http::Method;

use crate::error::Error;
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
