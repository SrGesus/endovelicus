use std::collections::BTreeMap;

use extism::{convert::Json, FromBytes};

#[derive(serde::Deserialize, serde::Serialize)]
pub enum HtmlType {
  #[serde(rename(serialize = "checkbox", deserialize = "checkbox"))]
  Checkbox,
  #[serde(rename(serialize = "color", deserialize = "color"))]
  Color,
  #[serde(rename(serialize = "date", deserialize = "date"))]
  Date,
  #[serde(rename(serialize = "datetime-local", deserialize = "datetime-local"))]
  DateTimeLocal,
  #[serde(rename(serialize = "email", deserialize = "email"))]
  Email,
  #[serde(rename(serialize = "file", deserialize = "file"))]
  File,
  #[serde(rename(serialize = "hidden", deserialize = "hidden"))]
  Hidden,
  #[serde(rename(serialize = "image", deserialize = "image"))]
  Image,
  #[serde(rename(serialize = "month", deserialize = "month"))]
  Month,
  #[serde(rename(serialize = "number", deserialize = "number"))]
  Number,
  #[serde(rename(serialize = "password", deserialize = "password"))]
  Password,
  #[serde(rename(serialize = "radio", deserialize = "radio"))]
  Radio,
  #[serde(rename(serialize = "range", deserialize = "range"))]
  Range,
  #[serde(rename(serialize = "reset", deserialize = "reset"))]
  Reset,
  #[serde(rename(serialize = "search", deserialize = "search"))]
  Search,
  #[serde(rename(serialize = "submit", deserialize = "submit"))]
  Submit,
  #[serde(rename(serialize = "tel", deserialize = "tel"))]
  Tel,
  #[serde(rename(serialize = "text", deserialize = "text"))]
  Text,
  #[serde(rename(serialize = "time", deserialize = "time"))]
  Time,
  #[serde(rename(serialize = "url", deserialize = "url"))]
  Url,
  #[serde(rename(serialize = "week", deserialize = "week"))]
  Week,
}

// Configuration
#[derive(serde::Serialize, serde::Deserialize, FromBytes)]
#[encoding(Json)]
pub struct ConfigVar {
  pub name: String,
  pub html_type: HtmlType,
  pub placeholder: String,
  pub required: bool,
}

#[derive(serde::Serialize, serde::Deserialize, FromBytes)]
#[encoding(Json)]
pub struct Config(pub BTreeMap<String, ConfigVar>);
