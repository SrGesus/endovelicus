mod json;
use axum::routing::{any, post, Router};
pub use json::*;

use crate::AppState;

pub mod account;
pub mod currency;
pub mod plugin;

pub fn router() -> Router<AppState> {
  Router::new()
    .route(
      "/currency",
      post(currency::create)
        .get(currency::read)
        .patch(currency::update)
        .delete(currency::delete),
    )
    // TODO: Finish account api
    .route(
      "/account",
      post(account::create)
        .get(account::read)
        .patch(account::update)
        .delete(account::delete),
    )
    .route(
      "/plugin",
      post(plugin::put).get(plugin::get).delete(plugin::delete),
    )
    .route("/plugin/:endpoint/:function", any(plugin::call))
  // TODO: Finish transaction api
  // TODO: Finish category api
}
