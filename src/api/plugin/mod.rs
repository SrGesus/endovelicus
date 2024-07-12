use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PLUGIN_FILE: &str = "plugins.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugins(HashMap<String, PluginData>);

mod api;
pub use api::*;
pub mod endpoint;

#[derive(Serialize, Deserialize)]
pub struct PluginData {
  #[serde(flatten)]
  wasm: Wasm,
  #[serde(skip)]
  plugin: Option<Plugin>,
}

impl Clone for PluginData {
  fn clone(&self) -> Self {
    Self {
      wasm: self.wasm.clone(),
      plugin: None,
    }
  }
}

impl Plugins {
  pub fn insert(&mut self, endpoint: String, wasm: Wasm) -> Option<PluginData> {
    self.0.insert(
      endpoint.clone(),
      PluginData {
        wasm: wasm.with_name(endpoint),
        plugin: None,
      },
    )
  }

  pub fn reload_plugins(&mut self) {
    for (_, data) in self.0.iter_mut() {
      data.plugin = None;
    }
  }

  pub fn get_plugin(&mut self, endpoint: &str) -> Option<&mut Plugin> {
    if let Some(plugin_data) = self.0.get_mut(endpoint) {
      if plugin_data.plugin.is_none() {
        plugin_data.plugin = Some(Plugin::new(Manifest::new([plugin_data.wasm.clone()]), [], true).unwrap());
      }
      plugin_data.plugin.as_mut()
    } else {
      None
    }
  }

  pub fn load() -> Self {
    match Self::load_file() {
      Ok(plugins) => plugins,
      Err(err) => {
        tracing::error!("Could not load {}: {}", PLUGIN_FILE, err);
        let mut h = HashMap::new();
        h.insert(
          "count".to_owned(),
          PluginData {
            wasm: Wasm::url(
              "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
            )
            .with_name("count"),
            plugin: None,
          },
        );
        Self(h)
      }
    }
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
