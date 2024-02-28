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
    "./src/abi/NonfungiblePositionManager.json",
);
// 订阅同质化合约地址mint 事件
pub async fn subscription_nonfungible_position_manager_mint(
    nonfungible_position_manager_address: Address,
    client: &Arc<Provider<Ws>>,
) -> eyre::Result<Mint> {
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
