use crate::data::account as data;
use axum::extract::State;
use entity::account;

use super::Json;
use crate::error::Error;

use crate::AppState;

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(account): Json<account::Model>,
) -> Result<Json<account::Model>, Error> {
  Ok(Json(data::insert(&database, account).await?))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  Json(account): Json<account::OptionalModel>,
) -> Result<Json<Vec<account::Model>>, Error> {
  Ok(Json(data::select(&database, account).await?))
}

pub async fn update(
  State(AppState(database, _)): State<AppState>,
  Json(account): Json<account::OptionalModel>,
) -> Result<Json<account::Model>, Error> {
  Ok(Json(data::update(&database, account).await?))
}

pub async fn delete(
  State(AppState(database, _)): State<AppState>,
  Json(account): Json<account::OptionalModel>,
) -> Result<&'static str, Error> {
  let name = account.name.clone();
  match data::remove(&database, account).await?.rows_affected {
    0 => Err(Error::NoSuchEntity("Account", "name", name.unwrap())),
    1 => Ok("Deleted account."),
    _ => unreachable!(),
  }
}
