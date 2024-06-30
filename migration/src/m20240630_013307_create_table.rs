use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    manager
      .create_table(
        Table::create()
          .table(Currency::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Currency::Code)
              .string_len(3u32)
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Currency::Name).string().not_null())
          .col(ColumnDef::new(Currency::Symbol).string().not_null())
          .col(ColumnDef::new(Currency::Rate).float().not_null())
          .to_owned(),
      )
      .await?;

    let insert = Query::insert()
      .into_table(Currency::Table)
      .columns(vec![
        Currency::Code,
        Currency::Name,
        Currency::Symbol,
        Currency::Rate,
      ])
      .values_panic(vec!["EUR".into(), "Euro".into(), "€".into(), "1.0".into()])
      .to_owned();

    manager.exec_stmt(insert).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    manager
      .drop_table(Table::drop().table(Currency::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Currency {
  Table,
  Code,
  Name,
  Symbol,
  Rate,
}
