use axum::extract::State;
use entity::currency;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;

use super::Error;
use super::Json;

use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OptionalModel {
  code: Option<String>,
  name: Option<String>,
  symbol: Option<String>,
  rate: Option<f64>,
}

impl OptionalModel {
  fn active(self) -> currency::ActiveModel {
    currency::ActiveModel {
      code: match self.code {
        Some(code) => Set(code),
        None => NotSet,
      },
      name: match self.name {
        Some(name) => Set(name),
        None => NotSet,
      },
      symbol: match self.symbol {
        Some(symbol) => Set(Some(symbol)),
        None => NotSet,
      },
      rate: match self.rate {
        Some(rate) => Set(rate),
        None => NotSet,
      },
    }
  }
}

async fn update_model(
  database: DatabaseConnection,
  currency: OptionalModel,
) -> Result<currency::Model, Error> {
  if currency.code.is_none() {
    return Err(Error::InvalidParameter("Currency code is required."));
  }
  currency
    .active()
    .update(&database)
    .await
    .map_err(|err| err.into())
}

async fn select_model(
  database: DatabaseConnection,
  currency: OptionalModel,
) -> Result<Vec<currency::Model>, Error> {
  let mut c = currency::Entity::find();
  if let Some(code) = &currency.code {
    c = c.filter(currency::Column::Code.eq(code));
  } else {
    if let Some(name) = &currency.name {
      c = c.filter(currency::Column::Name.contains(name));
    }
    if let Some(symbol) = &currency.symbol {
      c = c.filter(currency::Column::Symbol.contains(symbol));
    }
    if let Some(rate) = &currency.rate {
      c = c.filter(currency::Column::Rate.eq(*rate));
    }
  }
  Ok(c.all(&database).await?)
}

async fn insert_model(
  database: DatabaseConnection,
  currency: currency::Model,
) -> Result<currency::Model, Error> {
  tracing::info!("Inserting currency: {:?}", currency);
  if currency.code.len() != 3 {
    return Err(Error::InvalidParameter(
      "Currency code must be 3 letters long.",
    ));
  }
  currency::ActiveModel {
    code: Set(currency.code.clone()),
    name: Set(currency.name),
    symbol: Set(currency.symbol),
    rate: Set(currency.rate),
  }
  .insert(&database)
  .await
  .map_err(|err| match err.sql_err() {
    Some(SqlErr::UniqueConstraintViolation(_)) => {
      Error::DuplicateEntity("Currency", "code", format!("'{}'.", currency.code))
    }
    _ => err.into(),
  })
}

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<currency::Model>,
) -> Result<Json<currency::Model>, Error> {
  Ok(Json(insert_model(database, payload).await?))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<OptionalModel>,
) -> Result<Json<Vec<currency::Model>>, Error> {
  let mut c = currency::Entity::find();
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
  Ok(Json(c.all(&database).await?))
}
