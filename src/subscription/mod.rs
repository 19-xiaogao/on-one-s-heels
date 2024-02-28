mod client;
mod factory;
mod nonfungible_position_manager;
pub(crate) use client::*;
pub(crate) use factory::*;
pub(crate) use nonfungible_position_manager::*;

// pub async fn _subscription_pool_swap(
//     pool_address: Address,
//     client: &Arc<Provider<Ws>>,
// ) -> eyre::Result<()> {
//     abigen!(
//         UniswapPool,
//         r#"[
//         event Swap(address indexed sender,address indexed recipient,int256 amount0,int256 amount1,uint160 sqrtPriceX96,uint128 liquidity,int24 tick)
//         function balance0() external view returns (uint256)
//         function balance1() external view returns (uint256)
//     ]"#,
//     );
//     let pool_contract = UniswapPool::new(pool_address, client.clone());

//     let events = pool_contract.event::<SwapFilter>().from_block(19117504);
//     let mut stream = events.stream().await?.take(1);
//     while let Some(Ok(f)) = stream.next().await {
//         println!("sender:{}", f.sender);
//         println!("recipient:{}", f.recipient);
//         println!("amount_0:{}", f.amount_0);
//         println!("amount_1::{}", f.amount_1);
//         println!("sqrt_price_x96:::{}", f.sqrt_price_x96);
//         println!("liquidity::::{}", f.liquidity);
//         println!("tick::::{}", f.tick);
//         println!("---------------------------------------------------------");
//     }

//     Ok(())
// }
