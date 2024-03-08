// 监听一个合约池子的swap事件,但价格合适的时候买入, 或者卖出。
use ethers::contract::abigen;
// use ethers::core::k256::sha2::digest::typenum::Pow;
// use ethers::core::k256::U256;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
use std::sync::Arc;

abigen!(V3_Pool, "./src/abi/v3_pool.json");

// #[derive(Debug, Default)]
// pub struct Swap {
//     sender: String,
//     recipient: String,
//     amount_0: i128,
//     amount_1: i128,
//     price: i128,
// }

pub async fn subscription_pool(
    pool_address: Address,
    start_block_number: u64,
    client: &Arc<Provider<Ws>>,
) {
    let pool_contract = V3_Pool::new(pool_address, client.into());
    let events = pool_contract
        .event::<SwapFilter>()
        .from_block(start_block_number);
    let mut stream = events.stream().await.unwrap().take(1);

    let mut price = "".to_string();

    while let Some(Ok(f)) = stream.next().await {
        println!("swapFilter event: {f:?}");
        price = f.sqrt_price_x96.to_string();
    }
    let price = sqrt_price_x96_to_price_ratio(price.parse().unwrap());
    println!("price,{}", price);
}

fn sqrt_price_x96_to_price_ratio(sqrt_price_x96: u128) -> f64 {
    let price_ratio = (sqrt_price_x96 as f64).powi(2) / (1u128 << 96) as f64;
    price_ratio
}
