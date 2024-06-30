use axum::extract::{Json, State};
use entity::currency;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn create(
  State(database): State<DatabaseConnection>,
  Json(payload): Json<currency::Model>,
) -> String {
  tracing::info!("Creating currency: {:?}", payload);
  let currency = currency::ActiveModel {
    code: Set(payload.code),
    name: Set(payload.name),
    symbol: Set(payload.symbol),
    rate: Set(payload.rate),
  }
  // .save(&database)
  .insert(&database)
  .await;
  match currency {
    Ok(_) => "Currency created".to_owned(),
    Err(err) => format!("Error creating currency: {}", err),
  }
}
