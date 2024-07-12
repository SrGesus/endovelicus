use anyhow::Context;
use extism::Wasm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PLUGIN_FILE: &str = "plugins.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugins(HashMap<String, Plugin>);

mod api;
pub use api::*;
pub mod endpoint;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugin {
  pub name: String,
  pub wasm: Wasm,
}

impl Plugins {
  pub fn load() -> Self {
    match Self::load_file() {
      Ok(plugins) => plugins,
      Err(err) => {
        tracing::error!("Could not load {}: {}", PLUGIN_FILE, err);
        let mut h = HashMap::new();
        h.insert(
          "banana".to_owned(),
          Plugin {
            name: "banana".to_owned(),
            wasm: Wasm::url(
              "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
            ),
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
