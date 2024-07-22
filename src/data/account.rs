use crate::error::Error;
use entity::account;
use sea_orm::{entity::prelude::*, DeleteResult, IntoActiveModel};

pub async fn insert(
  database: &DatabaseConnection,
  account: account::Model,
) -> Result<account::Model, Error> {
  tracing::info!("Inserting account: {:?}", account);
  let name = account.name.clone();
  let code = account.currency.clone();
  account
    .into_active_model()
    .insert(database)
    .await
    .map_err(|err| match err.sql_err() {
      Some(SqlErr::UniqueConstraintViolation(_)) => Error::DuplicateEntity("Account", "name", name),
      Some(SqlErr::ForeignKeyConstraintViolation(_)) => {
        Error::NoSuchEntity("Currency", "code", code)
      }
      _ => err.into(),
    })
}

pub async fn select(
  database: &DatabaseConnection,
  account: account::OptionalModel,
) -> Result<Vec<account::Model>, Error> {
  let mut a = account::Entity::find();
  if let Some(name) = &account.name {
    a = a.filter(account::Column::Name.contains(name));
  }
  if let Some(r#type) = &account.r#type {
    a = a.filter(account::Column::Name.contains(r#type));
  }
  if let Some(currency) = &account.currency {
    a = a.filter(account::Column::Name.eq(currency));
  }
  Ok(a.all(database).await?)
}

pub async fn update(
  database: &DatabaseConnection,
  account: account::OptionalModel,
) -> Result<account::Model, Error> {
  if account.name.is_none() {
    return Err(Error::InvalidParameter("Account name is required."))
  }
  Ok(account.into_active().update(database).await?)
}

pub async fn delete(
  database: &DatabaseConnection,
  account: account::OptionalModel,
) -> Result<DeleteResult, Error> {
  if account.name.is_none() {
    return Err(Error::InvalidParameter("Account name is required."))
  }
  Ok(account.into_active().delete(database).await?)
}
