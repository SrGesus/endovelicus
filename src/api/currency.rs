use axum::extract::{Json, State};
use axum::http::StatusCode;
use entity::currency;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
  code: Option<String>,
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
    Err(err) => format!("Error creating currency: {err}"),
  }
}

pub async fn read(
  State(database): State<DatabaseConnection>,
  payload: Option<Json<Input>>,
) -> Result<Json<Vec<currency::Model>>, StatusCode> {
  let mut c = currency::Entity::find();
  if let Some(Json(payload)) = payload {
    if let Some(code) = &payload.code {
      c = c.filter(currency::Column::Code.eq(code));
    } else {
      if let Some(name) = &payload.name {
        c = c.filter(currency::Column::Name.eq(name));
      }
      if let Some(symbol) = &payload.symbol {
        c = c.filter(currency::Column::Symbol.eq(symbol));
      }
      if let Some(rate) = &payload.rate {
        c = c.filter(currency::Column::Rate.eq(*rate));
      }
    }
  }
  Ok(Json(
    c.all(&database)
      .await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
  ))
}
