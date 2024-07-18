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
          .col(ColumnDef::new(Currency::Symbol).string())
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

    manager
      .create_table(
        Table::create()
          .table(Account::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Account::Name)
              .string()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Account::Type).string().not_null())
          .col(
            ColumnDef::new(Account::Currency)
              .string_len(3u32)
              .not_null(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk_account_currency")
              .from(Account::Table, Account::Currency)
              .to(Currency::Table, Currency::Code),
          )
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts
    manager
      .drop_table(Table::drop().table(Account::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Currency::Table).to_owned())
      .await?;
    Ok(())
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

#[derive(DeriveIden)]
enum Account {
  Table,
  Name,
  Type,
  Currency,
}
