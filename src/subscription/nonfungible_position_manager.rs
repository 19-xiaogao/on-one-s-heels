use chrono::{Local, NaiveDateTime};
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
use ethers::types::U256;
use ethers::utils::hex::ToHexExt;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{logging, models};

use super::get_pool_address;
#[derive(Debug, Deserialize, Serialize)]
pub struct Mint {
    pub token_id: String,
    pub liquidity: String,
    pub amount0: String,
    pub amount1: String,
}

abigen!(
    NonfungiblePositionManager,
    "./src/abi/NonfungiblePositionManager.json"
);

// subscription nonfungible contract mint event  and insert database data
pub async fn subscription_nonfungible_position_manager_mint(
    nonfungible_position_manager_address: Address,
    factory_address: Address,
    client: &Arc<Provider<Ws>>,
    db: &DatabaseConnection,
) {
    let nonfungible_contract =
        NonfungiblePositionManager::new(nonfungible_position_manager_address, client.clone());
    let events = nonfungible_contract
        .event::<IncreaseLiquidityFilter>()
        .from_block(4734414);
    let mut stream = events.stream().await.unwrap().take(1);
    let mut mint = Mint {
        token_id: Default::default(),
        liquidity: Default::default(),
        amount0: Default::default(),
        amount1: Default::default(),
    };

    while let Some(Ok(f)) = stream.next().await {
        println!("IncreaseLiquidityFilter event: {f:?}");

        mint.token_id = f.token_id.to_string();
        mint.liquidity = f.liquidity.to_string();
        mint.amount0 = f.amount_0.to_string();
        mint.amount1 = f.amount_1.to_string();
    }

    if let Ok(positions) = nonfungible_contract
        .positions(U256::from_dec_str(mint.token_id.as_str()).unwrap())
        .call()
        .await
    {
        println!("positions: {:?}", positions);
        let pool_address = get_pool_address(
            factory_address,
            &client.clone(),
            positions.2,
            positions.3,
            positions.4,
        )
        .await
        .unwrap();
        let create_time = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0).unwrap();
        let update_time = create_time.clone();
        let pool_detail_result = models::query_pool_detail_for_token_id(db, mint.token_id.clone())
            .await
            .unwrap();

        let change_model = models::PoolDetailModel {
            token_id: mint.token_id.parse().unwrap(),
            amount0: mint.amount0,
            amount1: mint.amount1,
            pool_address: pool_address.encode_hex_with_prefix(),
            nonce: positions.0 as i32,
            operator: positions.1.encode_hex_with_prefix(),
            token0: positions.2.encode_hex_with_prefix(),
            token1: positions.3.encode_hex_with_prefix(),
            fee: positions.4 as i32,
            tick_lower: positions.5 as i32,
            tick_upper: positions.6 as i32,
            liquidity: positions.7.to_string(),
            fee_growth_inside0_last_x128: (positions.8).to_string(),
            fee_growth_inside1_last_x128: positions.9.to_string(),
            tokens_owed0: positions.10 as i32,
            tokens_owed1: positions.11 as i32,
            create_time,
            update_time,
            id: Default::default(),
        };

        if pool_detail_result.len() <= 0 {
            models::insert_pool_detail(db, change_model)
                .await
                .unwrap_or_else(|err| {
                    logging::log_error(&err.to_string());
                    println!("insert pool err :{}", err);
                });
        } else {
            models::update_pool_detail(db, mint.token_id.clone(), change_model)
                .await
                .unwrap_or_else(|err| {
                    logging::log_error(&err.to_string());
                    println!("insert pool err :{}", err);
                });
        }
    }
}
