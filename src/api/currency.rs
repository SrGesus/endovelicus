use crate::models::currency;
use axum::extract::{Json, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Currency {
  code: String,
  name: String,
  symbol: String,
  rate: f32,
}

pub async fn create(
  State(database): State<DatabaseConnection>,
  Json(payload): Json<Currency>,
) -> String {
  let currency = currency::ActiveModel {
    code: Set(payload.code),
    name: Set(payload.name),
    symbol: Set(payload.symbol),
    rate: Set(payload.rate),
  }
  .save(&database)
  .await;
  match currency {
    Ok(_) => "Currency created".to_owned(),
    Err(err) => format!("Error creating currency: {}", err),
  }
}
