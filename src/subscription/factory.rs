use chrono::{Local, NaiveDateTime};
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
use ethers::utils::hex::ToHexExt;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{logging, models};

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolCreate {
    pub token0: Address,
    pub token1: Address,
    pub fee: u16,
    pub tick_spacing: i16,
    pub pool_address: Address,
}

abigen!(
    Factory,
    r#"[
        event PoolCreated(address indexed token0, address indexed token1,uint24 indexed fee,int24 tickSpacing,address pool)
        function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool)
    ]"#,
);

pub async fn get_pool_address(
    factory_address: Address,
    client: &Arc<Provider<Ws>>,
    token_a: Address,
    token_b: Address,
    fee: u32,
) -> Result<Address, ()> {
    let factory_contract = Factory::new(factory_address, client.clone());

    let pool_address = factory_contract
        .get_pool(token_a, token_b, fee)
        .call()
        .await
        .unwrap();
    Ok(pool_address as Address)
}

// 订阅池子创建
pub async fn subscription_factory_pool_create(
    factory_address: Address,
    client: &Arc<Provider<Ws>>,
    db: &DatabaseConnection,
) {
    let factory_contract = Factory::new(factory_address, client.clone());
    let events = factory_contract
        .event::<PoolCreatedFilter>()
        .from_block(12369621);
    let mut stream = events.stream().await.unwrap().take(1);

    let mut pool_create: PoolCreate = PoolCreate {
        token0: Default::default(),
        token1: Default::default(),
        fee: 0,
        tick_spacing: 0,
        pool_address: Default::default(),
    };

    while let Some(Ok(val)) = stream.next().await {
        pool_create.token0 = val.token_0;
        pool_create.token1 = val.token_1;
        pool_create.fee = val.fee as u16;
        pool_create.tick_spacing = val.tick_spacing as i16;
        pool_create.pool_address = val.pool;
    }

    let create_time = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0).unwrap();

    let update_time = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(),0).unwrap()
    models::insert_pool(
        &db,
        models::Model {
            id: Default::default(),
            token0: pool_create.token0.encode_hex_with_prefix(),
            token1: pool_create.token1.encode_hex_with_prefix(),
            pool_address: pool_create.pool_address.encode_hex_with_prefix(),
            fee: pool_create.fee as i32,
            tick_spacing: pool_create.tick_spacing as i32,
            create_time,
        },
    )
    .await
    .unwrap_or_else(|err| {
        logging::log_error(&err.to_string());
        println!("insert pool err :{}", err);
    });
}
