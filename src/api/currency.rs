use axum::extract::{Json, State};
use entity::currency;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Input {
  code: Option<String>,
  name: Option<String>,
  symbol: Option<String>,
  rate: Option<f64>,
}

pub async fn create(
  State(database): State<DatabaseConnection>,
  Json(payload): Json<currency::Model>,
) -> String {
  tracing::info!("Creating currency: {:?}", payload);
  let currency = currency::ActiveModel {
    code: Set(payload.code),
    name: Set(payload.name),
    symbol: Set(payload.symbol),
    rate: Set(payload.rate),
  }
  .insert(&database)
  .await;
  match currency {
    Ok(_) => "Currency created".to_owned(),
    Err(err) => format!("Error creating currency: {err}"),
  }
}

pub async fn read(
  State(database): State<DatabaseConnection>,
  payload: Option<Json<Input>>,
) -> Json<Vec<currency::Model>> {
  match payload {
    Some(Json(payload)) => match payload.code {
      Some(code) => Json(
        currency::Entity::find_by_id(code)
          .all(&database)
          .await
          .unwrap(),
      ),
      None => {
        let mut c = currency::Entity::find();
        if let Some(name) = &payload.name {
          c = c.filter(currency::Column::Name.contains(name));
        }
        if let Some(symbol) = &payload.symbol {
          c = c.filter(currency::Column::Symbol.contains(symbol));
        }
        if let Some(rate) = &payload.rate {
          c = c.filter(currency::Column::Rate.eq(*rate));
        }
        Json(c.all(&database).await.unwrap())
      }
    },
    None => {
      let currencies = currency::Entity::find().all(&database).await.unwrap();
      Json(currencies)
    }
  }
}
