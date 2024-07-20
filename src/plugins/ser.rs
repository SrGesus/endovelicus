use futures::{stream::FuturesUnordered, StreamExt};
use serde::{Deserialize, Deserializer, Serialize};
/// Serde serialization and deserialization for [`PluginStore`]
use std::collections::BTreeMap;
use std::sync::Arc;

use super::{PluginData, PluginStore};

#[derive(Serialize, Deserialize)]
pub struct SerPluginStore(pub BTreeMap<String, PluginData>);

impl PluginStore {
  /// Returns a copy of [`PluginStore`] as a [`SerPluginStore`] which implements
  /// Serialize, requires getting a read lock on all plugins in the
  /// [`PluginStore`].
  pub async fn to_serializable(&self) -> SerPluginStore {
    SerPluginStore(
      self
        .iter()
        .map(|(k, v)| async { (k.clone(), v.read().await.clone()) })
        .collect::<FuturesUnordered<_>>()
        .collect()
        .await,
    )
  }
}

impl Into<PluginStore> for SerPluginStore {
  fn into(self) -> PluginStore {
    PluginStore(
      self
        .0
        .into_iter()
        .map(|(k, v)| (k, Arc::new(tokio::sync::RwLock::new(v))))
        .collect(),
    )
  }
}

impl<'de> Deserialize<'de> for PluginStore {
  fn deserialize<D>(deserializer: D) -> Result<PluginStore, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(SerPluginStore::deserialize(deserializer)?.into())
  }
}
