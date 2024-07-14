use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
// use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

const PLUGIN_FILE: &str = "plugins.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugins(HashMap<String, Arc<RwLock<PluginData>>>);

mod api;
pub use api::*;
pub mod endpoint;

#[derive(Serialize, Deserialize)]
pub struct PluginData {
  #[serde(flatten)]
  wasm: Wasm,
  #[serde(skip)] // This needs to be generated during runtime
  plugin: Option<Plugin>,
  config: Option<BTreeMap<String, String>>,
}

impl PluginData {
  pub fn plugin_mut(&mut self) -> &mut Plugin {
    if self.plugin.is_none() {
      let mut manifest = Manifest::new([self.wasm.clone()]);
      if let Some(config) = &self.config {
        manifest.config = config.clone();
      }
      self.plugin = Plugin::new(manifest, [], true).ok();
    }
    self.plugin.as_mut().unwrap() // Safe to unwrap since we just set it
  }
}

impl Default for Plugins {
  fn default() -> Self {
    let mut h = HashMap::new();
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

// FIXME: Find a way to avoid cloning for every read request
// even if PluginData is not big
impl Clone for PluginData {
  fn clone(&self) -> Self {
    Self {
      wasm: self.wasm.clone(),
      plugin: None,
      config: self.config.clone(),
    }
  }
}

impl Plugins {
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

  pub fn reload_plugins(&mut self) {
    for (_, data) in self.0.iter_mut() {
      // Since this requires a mutable reference to the map, getting a lock on every plugin is easy
      // Although it would be better to not get a lock on every plugin at all
      data.write().unwrap().plugin = None;
    }
  }

  pub fn get<'a>(&'a self, endpoint: &str) -> Option<RwLockReadGuard<'a, PluginData>> {
    if let Some(plugin_data) = self.0.get(endpoint) {
      Some(plugin_data.read().unwrap())
    } else {
      None
    }
  }

  pub fn get_mut<'a>(&'a self, endpoint: &str) -> Option<RwLockWriteGuard<'a, PluginData>> {
    if let Some(plugin_data) = self.0.get(endpoint) {
      Some(plugin_data.write().unwrap())
    } else {
      None
    }
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

  pub fn save(&self) {
    let plugins = serde_json::to_string_pretty(&self).expect("Failed to serialize plugins.");
    std::fs::write(PLUGIN_FILE, plugins).expect("Failed to write to plugins file.");
  }
}
