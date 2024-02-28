use chrono::{Local, NaiveDateTime};
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
use ethers::types::{U128, U256};
use ethers::utils::hex::ToHexExt;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{logging, models};
#[derive(Debug, Deserialize, Serialize)]
pub struct Mint {
    pub token_id: U256,
    pub liquidity: U128,
    pub amount0: U256,
    pub amount1: U256,
}

abigen!(
    NonfungiblePositionManager,
    r#"[
    event IncreaseLiquidity(uint256 tokenId,uint128 liquidity,uint256 amount0,uint256 amount1) 
    function positions(uint256 tokenId) external view override returns (uint96 nonce,address operator,address token0,address token1,uint24 fee,int24 tickLower,int24 tickUpper,uint128 liquidity,uint256 feeGrowthInside0LastX128,uint256 feeGrowthInside1LastX128,uint128 tokensOwed0,uint128 tokensOwed1) 
    ]"#,
);

// 订阅同质化合约地址mint 事件
pub async fn subscription_nonfungible_position_manager_mint(
    nonfungible_position_manager_address: Address,
    client: &Arc<Provider<Ws>>,
) -> eyre::Result<Mint> {
    let nonfungible_contract =
        NonfungiblePositionManager::new(nonfungible_position_manager_address, client.clone());
    let events = nonfungible_contract
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

    if let Ok(positions) = nonfungible_contract.positions(mint.token_id).call().await {
        println!("positions: {:?}", positions);
    }

    let create_time = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0).unwrap();
    let update_time = create_time.clone();
    // models:
    Ok(mint)
}
