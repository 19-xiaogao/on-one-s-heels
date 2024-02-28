use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pool_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub token_id: i32,
    pub liquidity: String,
    pub amount0: String,
    pub amount1: String,
    pub operator: String,
    pub pool_address: String,
    pub nonce: i32,
    pub token0: String,
    pub token1: String,
    pub fee: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub fee_growth_inside0_last_x128: String,
    pub fee_growth_inside1_last_x128: String,
    pub tokens_owed0: i32,
    pub tokens_owed1: i32,
    pub create_time: DateTime,
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn insert_pool_detail(db: &DbConn, from_data: Model) -> Result<(), DbErr> {
    ActiveModel {
        id: NotSet,
        token_id: Set(from_data.token_id.to_owned()),
        amount0: Set(from_data.amount0.to_owned()),
        amount1: Set(from_data.amount1.to_owned()),
        liquidity: Set(from_data.liquidity.to_owned()),
        operator: Set(from_data.operator.to_owned()),
        pool_address: Set(from_data.pool_address.to_owned()),
        nonce: Set(from_data.nonce.to_owned()),
        token0: Set(from_data.token0.to_owned()),
        token1: Set(from_data.token1.to_owned()),
        fee: Set(from_data.fee.to_owned()),
        tick_lower: Set(from_data.tick_lower.to_owned()),
        tick_upper: Set(from_data.tick_upper.to_owned()),
        fee_growth_inside0_last_x128: Set(from_data.fee_growth_inside0_last_x128.to_owned()),
        fee_growth_inside1_last_x128: Set(from_data.fee_growth_inside1_last_x128.to_owned()),
        tokens_owed0: Set(from_data.tokens_owed0.to_owned()),
        tokens_owed1: Set(from_data.tokens_owed1.to_owned()),
        create_time: Set(from_data.create_time.to_owned()),
        update_time: Set(from_data.update_time.to_owned()),
    }
    .insert(db)
    .await?;
    Ok(())
}

pub async fn query_pool_detail_for_token_id(
    db: &DbConn,
    token_id: u64,
) -> Result<Vec<Model>, DbErr> {
    let result = Entity::find()
        .filter(Column::TokenId.contains(token_id.to_string()))
        .all(db)
        .await?;
    Ok(result)
}

pub async fn update_pool_detail(db: &DbConn, token_id: u64, from_data: Model) -> Result<(), DbErr> {

    Ok(())
}
