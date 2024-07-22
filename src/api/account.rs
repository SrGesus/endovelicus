use crate::data::account::*;
use axum::extract::State;
use entity::account;

use super::Json;
use crate::error::Error;

use axum::http::StatusCode;
use sea_orm::entity::prelude::*;

use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub currency: Option<String>,
}

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(account): Json<account::Model>,
) -> Result<Json<account::Model>, Error> {
  Ok(Json(insert(&database, account).await?))
}

pub async fn read(
  State(AppState(database, _)): State<AppState>,
  payload: Option<Json<Input>>,
) -> Result<Json<Vec<account::Model>>, StatusCode> {
  let mut a = account::Entity::find();
  if let Some(Json(payload)) = payload {
    if let Some(name) = &payload.name {
      a = a.filter(account::Column::Name.contains(name));
    }
    if let Some(r#type) = &payload.r#type {
      a = a.filter(account::Column::Type.eq(r#type));
    }
    if let Some(currency) = &payload.currency {
      a = a.filter(account::Column::Currency.eq(currency));
    }
  }
  Ok(Json(
    a.all(&database)
      .await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
  ))
}
