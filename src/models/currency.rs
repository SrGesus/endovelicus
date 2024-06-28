
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "currency")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Char(3)")]
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub rate: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {

}

impl ActiveModelBehavior for ActiveModel {}
