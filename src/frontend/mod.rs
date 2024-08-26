use axum::{debug_handler, extract::{Path, State}, response::Html};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct Route<'a> {
  title: &'a str,
  route: &'a str,
  icon: &'a str
}

#[debug_handler]
pub async fn frontend(State(_): State<AppState>, Path(path): Path<String>) -> Html<String> {

  let tera = match Tera::new("templates/**/*.tera") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error(s): {}", e);
      ::std::process::exit(1);
    }
  };
  let mut context = Context::new();
  let routes = [
    Route {
      title: "Accounts",
      route: "/account",
      icon: "mdi-account"
    },
    Route {
      title: "Transactions",
      route: "/transaction",
      icon: "mdi-cash-fast"
    },
    Route {
      title: "Currencies",
      route: "/currency",
      icon: "mdi-currency-eur"
    },
    Route {
      title: "Plugins",
      route: "/plugin",
      icon: "mdi-puzzle"
    },
    Route {
      title: "Settings",
      route: "/settings",
      icon: "mdi-cog"
    },
  ];
  context.insert("routes", &routes);
  
  println!("{:?}", format!("{path}/index.html.tera"));
  Html(tera.render(format!("{path}/index.html.tera").as_str(), &context).unwrap())

}


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
  let mut context = Context::new();
  let routes = [
    Route {
      title: "Accounts",
      route: "/accounts",
      icon: "mdi-account"
    },
    Route {
      title: "Transactions",
      route: "/transactions",
      icon: "mdi-cash-fast"
    },
    Route {
      title: "Currencies",
      route: "/currency",
      icon: "mdi-currency-eur"
    },
    Route {
      title: "Plugins",
      route: "/plugins",
      icon: "mdi-puzzle"
    },
    Route {
      title: "Settings",
      route: "/settings",
      icon: "mdi-cog"
    },
  ];
  context.insert("routes", &routes);

  Html(tera.render("currency/index.html.tera", &context).unwrap())
}

