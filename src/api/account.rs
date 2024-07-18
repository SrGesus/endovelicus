use axum::extract::{Json, State};
use axum::http::StatusCode;
use entity::account;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;

use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub currency: Option<String>,
}

pub async fn create(
  State(AppState(database, _)): State<AppState>,
  Json(payload): Json<account::Model>,
) -> String {
  tracing::info!("Creating account: {}", payload.name);
  let account = account::ActiveModel {
    name: Set(payload.name),
    r#type: Set(payload.r#type),
    currency: Set(payload.currency),
  };
  // FIXME needs rewrite
  match account.insert(&database).await {
    Ok(_) => "Account created".to_owned(),
    Err(err) => format!("Error creating account: {err}"),
  }
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
