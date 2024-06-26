use sqlx::{
    any::{install_default_drivers, AnyArguments, AnyRow}, query::{self, Query}, sqlite::SqliteConnectOptions, AnyPool, Arguments, Database, IntoArguments
};

use std::env;

pub struct DataBase {
    pool: sqlx::AnyPool,
}

const DEFAULT_DB: &str = "sqlite:./db.sqlite";

impl DataBase {
    pub async fn change_db(&mut self, url: &str) -> Result<(), sqlx::Error> {
        self.pool = AnyPool::connect(url).await?;
        Ok(())
    }

    pub async fn new() -> Result<Self, sqlx::Error> {
        install_default_drivers();
        let url = env::var("DATABASE_URL").unwrap_or(DEFAULT_DB.to_string());
        let pool: AnyPool = AnyPool::connect(&url)
            .await
            .unwrap_or(AnyPool::connect(DEFAULT_DB).await?);
        Ok(DataBase { pool })
    }

    pub async fn query<'q> (
        &self,
        query: Query<'q, sqlx::any::Any, sqlx::any::AnyArguments<'q, >>,
    ) -> Result<Vec<AnyRow>, sqlx::Error> {
        query.fetch_all(&self.pool).await
    }

    pub async fn init(&self) {
        let query: Query<'_, sqlx::Any, AnyArguments<'_>> = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS currency (
              id SERIAL PRIMARY KEY
            )",
        );
        self.query(query).await.unwrap();

    //   self.conn.execute(
    //     "CREATE TABLE IF NOT EXISTS currency (
    //       id SERIAL PRIMARY KEY,
    //       name TEXT NOT NULL,
    //       symbol CHAR(3) NOT NULL,
    //     )",
    //     [],
    //   );
    //   self.conn.execute(
    //     "CREATE TABLE IF NOT EXISTS account (
    //       id SERIAL PRIMARY KEY,
    //       name TEXT NOT NULL,
    //       type TEXT NOT NULL,
    //       currency TEXT NOT NULL REFERENCES currency(name),
    //     )",
    //     [],
    //   );
    }
}
