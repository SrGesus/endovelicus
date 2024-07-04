use axum::{
  routing::{get, post},
  Router,
};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
mod api;
use api::plugin::Plugins;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
struct AppState(DatabaseConnection, Arc<RwLock<Plugins>>);

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let plugins = Plugins::load();
  plugins.save();

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
    .route("/account", post(api::account::create))
    .route("/account", get(api::account::read))
    .route("/plugin", post(api::plugin::create))
    .route("/plugin", get(api::plugin::read))
    .with_state(state);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
