use futures::future::BoxFuture;
use sqlx::{database::HasArguments, query::Query, Error};

const DEFAULT_DB: &str = "sqlite:db.sqlite";

mod sqlite;
pub use sqlite::SqliteDB;

pub async fn get_database() -> Result<Box<dyn Db<impl sqlx::Database>>, Error> {
  let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DB.to_string());
  Ok(Box::new(SqliteDB::new(&url).await?))
}

pub trait Db<DB: sqlx::Database> {
  fn query<'q>(
    &'q self,
    query: Query<'q, DB, <DB as HasArguments<'q>>::Arguments>,
    // ) -> Pin<Box<dyn futures::Future<Output = Result<Vec<DB::Row>, Error>> + Send + 'q>>;
  ) -> BoxFuture<Result<Vec<DB::Row>, Error>>;

  fn init(&self) -> BoxFuture<Result<(), Error>>;
}
