use chrono::{Local, NaiveDateTime};
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
use ethers::types::{U128, U256};
use ethers::utils::hex::ToHexExt;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
mod client;

pub(crate) use client::*;

use crate::{logging, models};

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolCreate {
    pub token0: Address,
    pub token1: Address,
    pub fee: u16,
    pub tick_spacing: i16,
    pub pool_address: Address,
}

// 订阅池子创建
pub async fn subscription_factory_pool_create(
    factory_address: Address,
    client: &Arc<Provider<Ws>>,
    db: &DatabaseConnection,
) {
    abigen!(
        Factory,
        r#"[
        event PoolCreated(address indexed token0, address indexed token1,uint24 indexed fee,int24 tickSpacing,address pool)
    ]"#,
    );

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Mint {
    pub token_id: U256,
    pub liquidity: U128,
    pub amount0: U256,
    pub amount1: U256,
}

// 订阅同质化合约地址mint 事件
pub async fn subscription_nonfungible_position_manager_mint(
    nonfungible_position_manager_address: Address,
    client: &Arc<Provider<Ws>>,
) -> eyre::Result<Mint> {
    abigen!(
        NonfungiblePositionManager,
        r#"[
        event IncreaseLiquidity(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1) 
    ]"#,
    );

    let factory_contract =
        NonfungiblePositionManager::new(nonfungible_position_manager_address, client.clone());
    let events = factory_contract
        .event::<IncreaseLiquidityFilter>()
        .from_block(12369621);
    let mut stream = events.stream().await?.take(1);

    let mut mint = Mint {
        token_id: Default::default(),
        liquidity: Default::default(),
        amount0: Default::default(),
        amount1: Default::default(),
    };

    while let Some(Ok(f)) = stream.next().await {
        mint.token_id = f.token_id;
        mint.liquidity = f.liquidity.into();
        mint.amount0 = f.amount_0;
        mint.amount1 = f.amount_1;
    }

    Ok(mint)
}

pub async fn _subscription_pool_swap(
    pool_address: Address,
    client: &Arc<Provider<Ws>>,
) -> eyre::Result<()> {
    abigen!(
        UniswapPool,
        r#"[
        event Swap(address indexed sender,address indexed recipient,int256 amount0,int256 amount1,uint160 sqrtPriceX96,uint128 liquidity,int24 tick)
        function balance0() external view returns (uint256)
        function balance1() external view returns (uint256)
    ]"#,
    );
    let pool_contract = UniswapPool::new(pool_address, client.clone());

    let events = pool_contract.event::<SwapFilter>().from_block(19117504);
    let mut stream = events.stream().await?.take(1);
    while let Some(Ok(f)) = stream.next().await {
        println!("sender:{}", f.sender);
        println!("recipient:{}", f.recipient);
        println!("amount_0:{}", f.amount_0);
        println!("amount_1::{}", f.amount_1);
        println!("sqrt_price_x96:::{}", f.sqrt_price_x96);
        println!("liquidity::::{}", f.liquidity);
        println!("tick::::{}", f.tick);
        println!("---------------------------------------------------------");
    }

    Ok(())
}
