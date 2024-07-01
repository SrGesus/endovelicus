use axum::extract::{Json, State};
use entity::account;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
    pub name: String,
    pub r#type: String,
    pub currency: String,
}

pub async fn create(
  State(database): State<DatabaseConnection>,
  Json(payload): Json<Input>,
) -> String {
  tracing::info!("Creating account: {}", payload.name);
  let account = account::ActiveModel {
    id: NotSet,
    name: Set(payload.name),
    r#type: Set(payload.r#type),
    currency: Set(payload.currency),
  };
  match account.insert(&database).await {
    Ok(_) => "Account created".to_owned(),
    Err(err) => format!("Error creating account: {}", err),
  }
}
