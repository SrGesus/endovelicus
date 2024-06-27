use futures::future::BoxFuture;
use sqlx::{database::HasArguments, query::Query, sqlite::SqliteConnectOptions, Error, Sqlite};
use std::str::FromStr;

use super::Db;

pub struct SqliteDB {
  pool: sqlx::sqlite::SqlitePool,
}

impl SqliteDB {
  pub async fn new(url: &str) -> Result<Self, Error> {
    let options = SqliteConnectOptions::from_str(url)
      .unwrap()
      .create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
      .connect_with(options)
      .await
      .unwrap();
    Ok(SqliteDB { pool })
  }
}

impl Db<Sqlite> for SqliteDB {
  fn query<'q>(
    &'q self,
    query: Query<'q, Sqlite, <Sqlite as HasArguments<'q>>::Arguments>,
  ) -> BoxFuture<Result<Vec<<Sqlite as sqlx::Database>::Row>, Error>> {
    Box::pin(query.fetch_all(&self.pool))
  }
  fn init(&self) -> BoxFuture<Result<(), Error>> {
    Box::pin(async {
      self
        .query(sqlx::query!(
          "CREATE TABLE currency (
            code CHAR(3) PRIMARY KEY,
            name TEXT NOT NULL,
            symbol TEXT NOT NULL,
            rate FLOAT NOT NULL,
            UNIQUE(name)
            );"
        ))
        .await?;
      self
        .query(sqlx::query!(
          "CREATE TABLE account (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            currency CHAR(3) NOT NULL REFERENCES currency(code)
          );"
        ))
        .await?;
      self
        .query(sqlx::query!(
          "CREATE TABLE IF NOT EXISTS transact (
            id SERIAL PRIMARY KEY,
            time TIMESTAMP NOT NULL,
            sender INTEGER REFERENCES account(id),
            amount_sent FLOAT,
            receiver INTEGER REFERENCES account(id),
            amount_received FLOAT,
            CONSTRAINT exchange_check CHECK (
                (amount_sent IS NULL) = (sender IS NULL) OR
                (amount_received IS NULL) = (receiver IS NULL)
            )
          );"
        ))
        .await?;
      Ok(())
    })
  }
}
