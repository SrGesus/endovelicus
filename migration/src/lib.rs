use sea_orm_migration::async_trait;
pub use sea_orm_migration::prelude::{MigrationTrait, MigratorTrait};

mod m20240630_013307_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20240630_013307_create_table::Migration)]
  }
}
