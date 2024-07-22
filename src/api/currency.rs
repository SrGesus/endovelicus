use crate::data::currency::*;
use axum::extract::State;
use entity::currency;

use super::Json;
use crate::error::Error;

use crate::AppState;

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<currency::Model>,
) -> Result<Json<currency::Model>, Error> {
  Ok(Json(insert(&database, payload).await?))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<currency::OptionalModel>,
) -> Result<Json<Vec<currency::Model>>, Error> {
  Ok(Json(select(&database, payload).await?))
}

pub async fn patch(
  State(AppState(database, _)): State<AppState>,
  Json(currency): Json<currency::OptionalModel>,
) -> Result<Json<currency::Model>, Error> {
  Ok(Json(update(&database, currency).await?))
}

pub async fn delete(
  State(AppState(database, _)): State<AppState>,
  Json(currency): Json<currency::OptionalModel>,
) -> Result<&'static str, Error> {
  let code = currency.code.clone();
  match remove(&database, currency).await?.rows_affected {
    0 => Err(Error::NoSuchEntity("Currency", "code", code.unwrap())),
    1 => Ok("Deleted currency."),
    _ => unreachable!(),
  }
}
