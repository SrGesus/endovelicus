mod export;
mod ser;
mod store;

use std::collections::BTreeMap;

use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

pub use export::*;
pub use ser::*;
pub use store::*;

use crate::error::Error;

#[derive(Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct PluginData {
  #[serde(flatten)]
  wasm: Wasm,
  #[serde(skip)] // This needs to be generated during runtime
  plugin: Option<Plugin>,
  config: Option<BTreeMap<String, String>>,
}

impl PluginData {
  pub fn new(wasm: Wasm, config: Option<BTreeMap<String, String>>) -> Self {
    let mut manifest = Manifest::new([wasm.clone()]).with_allowed_host("*");
    if let Some(ref config) = config {
      manifest.config = config.clone();
    }
    let plugin = Plugin::new(manifest, [], true).ok();
    PluginData {
      wasm,
      plugin,
      config,
    }
  }

  pub async fn config(&mut self) -> Result<Option<Config>, Error> {
    self
      .plugin_mut()
      .call("config", "")
      .map_err(|err| Error::Plugin(err))
      .map(|cfg: Option<Config>| cfg.filter(|cfg| !cfg.0.is_empty()))
  }

  pub fn plugin_mut(&mut self) -> &mut Plugin {
    if self.plugin.is_none() {
      let mut manifest = Manifest::new([self.wasm.clone()]).with_allowed_host("*");
      if let Some(config) = &self.config {
        manifest.config = config.clone();
      }
      // FIXME: Due to this lazy loading is off, plugin validity should be evaluated on creation
      self.plugin = Some(Plugin::new(manifest, [], true).unwrap());
    }
    self.plugin.as_mut().unwrap() // Safe to unwrap since we just set it
  }
}

impl Clone for PluginData {
  fn clone(&self) -> Self {
    Self {
      wasm: self.wasm.clone(),
      plugin: None,
      config: self.config.clone(),
    }
  }
}
