use axum::extract::State;
use entity::currency;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

use super::Error;
use super::Json;

use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct InputModel {
  code: Option<String>,
  name: Option<String>,
  symbol: Option<String>,
  rate: Option<f64>,
}

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<currency::Model>,
) -> Result<Json<currency::Model>, Error> {
  tracing::info!("Creating currency: {:?}", payload);

  if payload.code.len() != 3 {
    return Err(Error::InvalidParameter(
      "Currency code must be 3 letters long.",
    ));
  }

  let currency = currency::ActiveModel {
    code: Set(payload.code.clone()),
    name: Set(payload.name),
    symbol: Set(payload.symbol),
    rate: Set(payload.rate),
  }
  .insert(&database)
  .await
  .map_err(|err| match err.sql_err() {
    Some(SqlErr::UniqueConstraintViolation(_)) => {
      Error::DuplicateEntity("Currency", "code", format!("'{}'.", payload.code))
    }
    _ => err.into(),
  })?;
  Ok(Json(currency))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  payload: Option<Json<InputModel>>,
) -> Result<Json<Vec<currency::Model>>, Error> {
  let mut c = currency::Entity::find();
  if let Some(Json(payload)) = payload {
    if let Some(code) = &payload.code {
      c = c.filter(currency::Column::Code.eq(code));
    } else {
      if let Some(name) = &payload.name {
        c = c.filter(currency::Column::Name.contains(name));
      }
      if let Some(symbol) = &payload.symbol {
        c = c.filter(currency::Column::Symbol.contains(symbol));
      }
      if let Some(rate) = &payload.rate {
        c = c.filter(currency::Column::Rate.eq(*rate));
      }
    }
  }
  Ok(Json(c.all(&database).await?))
}
