use extism::Wasm;
use futures::future::join_all;
use futures::stream::{Collect, FuturesUnordered};
use futures::StreamExt;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::PluginData;
const PLUGIN_FILE: &str = "plugins.json";

// FIXME: remove pub
pub struct PluginStore(pub BTreeMap<String, Arc<RwLock<PluginData>>>);

impl Default for PluginStore {
  fn default() -> Self {
    let mut h = BTreeMap::new();
    h.insert(
      "count".to_owned(),
      Arc::new(RwLock::new(PluginData {
        wasm: Wasm::url(
          "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
        )
        .with_name("count"),
        plugin: None,
        config: None,
      })),
    );
    Self(h)
  }
}

impl PluginStore {
  pub fn insert(
    &mut self,
    endpoint: String,
    wasm: Wasm,
    config: Option<BTreeMap<String, String>>,
  ) -> Option<Arc<RwLock<PluginData>>> {
    self.0.insert(
      endpoint.clone(),
      Arc::new(RwLock::new(PluginData {
        wasm: wasm.with_name(endpoint),
        plugin: None,
        config,
      })),
    )
  }

  pub async fn reload_plugins(&self) {
    let futures = self
      .0
      .values()
      .map(|p| async { p.write().await.plugin = None });
    join_all(futures).await;
  }

  pub fn load() -> Self {
    Self::load_file().unwrap_or_else(|err| {
      tracing::error!("Failed to load plugins: {}", err);
      Self::default()
    })
  }

  fn load_file() -> Result<Self, anyhow::Error> {
    let plugins = std::fs::read_to_string(PLUGIN_FILE)?;
    Ok(serde_json::from_str(&plugins)?)
  }

  pub async fn save(&self) {
    let plugins = serde_json::to_string_pretty(&self.to_serializable().await)
      .expect("Failed to serialize plugins.");
    std::fs::write(PLUGIN_FILE, plugins).expect("Failed to write to plugins file.");
  }

  pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, Arc<RwLock<PluginData>>> {
    self.0.iter()
  }

  pub fn iter_mut(
    &mut self,
  ) -> std::collections::btree_map::IterMut<'_, String, Arc<RwLock<PluginData>>> {
    self.0.iter_mut()
  }
}
