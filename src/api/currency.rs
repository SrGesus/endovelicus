use axum::extract::{Query, State};
use entity::currency;

use super::Json;
use crate::data::currency as data;
use crate::error::Error;
use crate::AppState;

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<currency::Model>,
) -> Result<Json<currency::Model>, Error> {
  Ok(Json(data::insert(&database, payload).await?))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  Query(payload): Query<currency::OptionalModel>,
) -> Result<Json<Vec<currency::Model>>, Error> {
  Ok(Json(data::select(&database, payload).await?))
}

pub async fn update(
  State(AppState(database, _)): State<AppState>,
  Json(currency): Json<currency::OptionalModel>,
) -> Result<Json<currency::Model>, Error> {
  Ok(Json(data::update(&database, currency).await?))
}

pub async fn delete(
  State(AppState(database, _)): State<AppState>,
  Query(currency): Query<currency::OptionalModel>,
) -> Result<&'static str, Error> {
  let code = currency.code.clone();
  match data::remove(&database, currency).await?.rows_affected {
    0 => Err(Error::NoSuchEntity("Currency", "code", code.unwrap())),
    1 => Ok("Deleted currency."),
    _ => unreachable!(),
  }
}
