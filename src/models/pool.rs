use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pool")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub token0: String,
    pub token1: String,
    pub pool_address: String,
    pub fee: i32,
    pub tick_spacing: i32,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn insert_pool(db: &DbConn, from_data: Model) -> Result<Model, DbErr> {
    ActiveModel {
        token0: Set(from_data.token0.to_owned()),
        token1: Set(from_data.token1.to_owned()),
        pool_address: Set(from_data.pool_address.to_owned()),
        fee: Set(from_data.fee.to_owned()),
        tick_spacing: Set(from_data.tick_spacing.to_owned()),
        create_time: Set(from_data.create_time.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
}
