use axum::extract::{Json, State};
use entity::currency;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CurrencyInput {
  code: String,
  name: Option<String>,
  symbol: Option<String>,
  rate: Option<f64>,
}

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
  .insert(&database)
  .await;
  match currency {
    Ok(_) => "Currency created".to_owned(),
    Err(err) => format!("Error creating currency: {}", err),
  }
}

pub async fn read(
  State(database): State<DatabaseConnection>,
  payload: Option<Json<CurrencyInput>>,
) -> Json<Vec<currency::Model>> {
  match payload {
    Some(Json(payload)) => {
      let currency = currency::Entity::find_by_id(payload.code)
        .one(&database)
        .await
        .unwrap();
      match currency {
        Some(currency) => Json(vec![currency]),
        None => Json(vec![]),
      }
    }
    None => {
      let currencies = currency::Entity::find().all(&database).await.unwrap();
      Json(currencies)
    }
  }
}
