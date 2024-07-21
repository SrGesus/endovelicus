#![allow(dead_code)]
use axum::{
  routing::{any, delete, get, patch, post, put},
  Router,
};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::sync::RwLock;

mod api;
pub(crate) mod data;
mod error;

mod plugins;

use plugins::PluginStore;

#[derive(Clone)]
// DatabaseConnection already has an Arc inside
struct AppState(DatabaseConnection, Arc<RwLock<PluginStore>>);
#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let plugins_path = std::env::var("PLUGIN_JSON").unwrap_or("plugins.json".to_string());

  let plugins =
    PluginStore::load(&plugins_path);
  plugins.save(&plugins_path).await;

  // Configure and initialize the database
  let conn = Database::connect(std::env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

  tracing::info!(
    "Connected to the database at {}",
    std::env::var("DATABASE_URL").unwrap()
  );

  Migrator::up(&conn, None).await.unwrap();

  let state = AppState(conn, Arc::new(RwLock::new(plugins)));

  let app = Router::new()
    .route("/currency", post(api::currency::create))
    .route("/currency", get(api::currency::read))
    .route("/currency", patch(api::currency::patch))
    .route("/currency", delete(api::currency::delete))
    .route("/account", post(api::account::create))
    .route("/account", get(api::account::read))
    .route("/plugin", put(api::plugin::put))
    .route("/plugin", get(api::plugin::get))
    // .route("/plugin", delete(api::plugin::delete))
    .route("/plugin/:endpoint/:function", any(api::plugin::call))
    .with_state(state);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
