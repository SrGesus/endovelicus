use extism::Wasm;
use futures::future::join_all;
use futures::stream::{Collect, FuturesUnordered};
use futures::StreamExt;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{PluginData, SerPluginStore};
const PLUGIN_FILE: &str = "plugins.json";

// FIXME: remove pub
pub struct PluginStore(BTreeMap<String, Arc<RwLock<PluginData>>>);

impl From<BTreeMap<String, Arc<RwLock<PluginData>>>> for PluginStore {
  fn from(value: BTreeMap<String, Arc<RwLock<PluginData>>>) -> Self {
    PluginStore(value)
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

  pub async fn search(&self, endpoint: &str) -> SerPluginStore {
    SerPluginStore(if let Some(plugin) = self.0.get(endpoint) {
      BTreeMap::from([(endpoint.to_owned(), plugin.read().await.clone())])
    } else {
      BTreeMap::new()
    })
  }

  pub fn get_plugin(&self, endpoint: &str) -> Option<Arc<RwLock<PluginData>>> {
    self.0.get(endpoint).map(Arc::clone)
  }

  pub fn remove(&mut self, endpoint: &str) -> Option<()> {
    self.0.remove(endpoint).map(|_: Arc<RwLock<PluginData>>| ())
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }

  pub async fn reload(&self) {
    let futures = self
      .0
      .values()
      .map(|p| async { p.write().await.plugin = None });
    join_all(futures).await;
  }

  pub fn load(path: &str) -> Self {
    Self::load_file(path).unwrap_or_else(|err: anyhow::Error| {
      tracing::error!("Failed to load plugins: {}", err);
      Self::default()
    })
  }

  fn load_file(path: &str) -> Result<Self, anyhow::Error> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
  }

  pub async fn save(&self, path: &str) {
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

  // FIXME remove commented code
  // pub fn into_iter(
  //   self,
  // ) -> std::collections::btree_map::IntoIter<
  //   std::string::String,
  //   Arc<tokio::sync::RwLock<PluginData>>,
  // > {
  //   self.0.into_iter()
  // }

  pub fn values(
    &self,
  ) -> std::collections::btree_map::Values<
    '_,
    std::string::String,
    Arc<tokio::sync::RwLock<PluginData>>,
  > {
    self.0.values()
  }

  pub fn values_mut(
    &mut self,
  ) -> std::collections::btree_map::ValuesMut<
    '_,
    std::string::String,
    Arc<tokio::sync::RwLock<PluginData>>,
  > {
    self.0.values_mut()
  }
}

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
