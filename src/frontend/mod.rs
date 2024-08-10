use axum::{debug_handler, extract::State, response::Html};
use tera::{Context, Tera};

use crate::AppState;

#[debug_handler]
pub async fn test(State(_): State<AppState>) -> Html<String> {
  // Use globbing
  let tera = match Tera::new("templates/**/*.tera") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      ::std::process::exit(1);
    }
  };
  let context = Context::new();

  Html(tera.render("currency/index.html.tera", &context).unwrap())
}

