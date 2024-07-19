use crate::error::Error;
use entity::currency;
use sea_orm::{entity::prelude::*, DeleteResult, IntoActiveModel};

pub async fn insert(
  database: DatabaseConnection,
  currency: currency::Model,
) -> Result<currency::Model, Error> {
  tracing::info!("Inserting currency: {:?}", currency);
  let code = currency.code.clone();
  if code.len() != 3 {
    return Err(Error::InvalidParameter(
      "Currency code must be 3 letters long.",
    ));
  }
  currency
    .into_active_model()
    .insert(&database)
    .await
    .map_err(|err| match err.sql_err() {
      Some(SqlErr::UniqueConstraintViolation(_)) => {
        Error::DuplicateEntity("Currency", "code", code)
      }
      _ => err.into(),
    })
}

pub async fn select(
  database: DatabaseConnection,
  currency: currency::OptionalModel,
) -> Result<Vec<currency::Model>, Error> {
  let mut c = currency::Entity::find();
  if let Some(code) = &currency.code {
    c = c.filter(currency::Column::Code.contains(code));
  }
  if let Some(name) = &currency.name {
    c = c.filter(currency::Column::Name.contains(name));
  }
  if let Some(Some(symbol)) = &currency.symbol {
    c = c.filter(currency::Column::Symbol.contains(symbol));
  }
  if let Some(rate) = &currency.rate {
    c = c.filter(currency::Column::Rate.eq(*rate));
  }
  Ok(c.all(&database).await?)
}

pub async fn update(
  database: DatabaseConnection,
  currency: currency::OptionalModel,
) -> Result<currency::Model, Error> {
  if currency.code.is_none() {
    return Err(Error::InvalidParameter("Currency code is required."));
  }
  Ok(currency.into_active().update(&database).await?)
}

pub async fn remove(
  database: DatabaseConnection,
  currency: currency::OptionalModel,
) -> Result<DeleteResult, Error> {
  if currency.code.is_none() {
    return Err(Error::InvalidParameter("Currency code is required."));
  }
  Ok(currency.into_active().delete(&database).await?)
}
