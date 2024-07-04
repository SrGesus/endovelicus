use extism::Wasm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const PLUGIN_FILE: &str = "plugins.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugins(HashMap<String, Plugin>);

mod api;
pub use api::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugin {
  pub name: String,
  pub wasm: Wasm,
}

impl Plugins {
  pub fn load() -> Self {
    let plugins = std::fs::read_to_string(PLUGIN_FILE).expect("Failed to read from plugins file.");
    serde_json::from_str(&plugins).expect("Failed to parse plugins file.")
  }
  pub fn insert(&mut self, endpoint: String, name: String, plugin: Wasm) {
    self.0.insert(endpoint, Plugin { name, wasm: plugin });
  }
  pub fn save(&self) {
    let plugins = serde_json::to_string_pretty(&self).expect("Failed to serialize plugins.");
    std::fs::write(PLUGIN_FILE, plugins).expect("Failed to write to plugins file.");
  }
}
