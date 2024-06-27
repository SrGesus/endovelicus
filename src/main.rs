use axum::{routing::get, Router};

pub mod api;
pub mod db;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  // Configure and initialize the database
  let db = db::get_database().await.unwrap();
  db.init().await.unwrap();

  let app = Router::new().route("/", get(root));

  println!("Hello, world!");
  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
  tracing::info!("Yippie!");
  "Hello world!"
}
