#![allow(dead_code)]
use axum::{http::{header::CONTENT_TYPE, Method}, Router};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;
mod api;
pub(crate) mod data;
mod error;

mod plugin;

use plugin::PluginStore;

#[derive(Clone)]
// DatabaseConnection already has an Arc inside
struct AppState(DatabaseConnection, Arc<RwLock<PluginStore>>);
#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let plugins_path = std::env::var("PLUGIN_JSON").unwrap_or("plugins.json".to_string());

  let plugins = PluginStore::load(&plugins_path);
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

  let api = api::router();

  let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
    // allow requests from any origin
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any);

  let app = Router::new()
    .nest("/api", api)
    .with_state(state)
    .layer(cors);

  // run our app with hyper, listening globally on port 3030
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
  info!("Serving endovelicus on http://localhost:{}/", 3030);
  axum::serve(listener, app).await.unwrap();
}
