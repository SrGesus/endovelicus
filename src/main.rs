use axum::{
  routing::{get, post},
  Router,
};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

mod api;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  dotenv().ok();
  // Configure and initialize the database
  let conn = Database::connect(std::env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();
  Migrator::up(&conn, None).await.unwrap();

  tracing::info!(
    "Connected to the database at {}",
    std::env::var("DATABASE_URL").unwrap()
  );

  let app = Router::new()
    .route("/", get(root))
    .route("/currency", post(api::currency::create))
    .route("/currency", get(api::currency::read))
    .with_state(conn);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
  tracing::info!("Yippie!");
  "Hello world!"
}
