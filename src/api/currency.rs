use crate::db::Db;

use axum::{extract::Json, handler::Handler, routing::post, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Currency {
  code: String,
  name: String,
  symbol: String,
  rate: f32,
}

pub async fn create_currency<'a, DB: sqlx::Database>(
  Json(payload): Json<Currency>,
  database: &'a dyn Db<DB>,
) -> &'static str
where
  std::string::String: sqlx::Encode<'a, DB>,
  std::string::String: sqlx::Type<DB>,
  f32: sqlx::Encode<'a, DB>,
  f32: sqlx::Type<DB>,
{
  // INSERT INTO currency VALUES ('EUR', 'Euro', '€', 1.0);
  let query = sqlx::query(
    "
      INSERT INTO currency VALUES ($1, $2, $3, $4);
    ",
  )
  .bind(payload.code)
  .bind(payload.name)
  .bind(payload.symbol)
  .bind(payload.rate);

  match database.query(query).await {
    Ok(_) => "Currency created!",
    Err(_) => "Failed to create currency!",
  }
}
