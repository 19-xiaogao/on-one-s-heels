use std::sync::Arc;
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, StreamExt, Ws};
mod client;
pub(crate) use client::*;

#[derive(Debug)]
pub struct PoolCreate {
    token0 : Address,
    token1 : Address,
    fee : u16,
    tick_spacing: i16,
    pool_address : Address
}

// 订阅池子创建
pub async fn subscription_factory_pool_create(factory_address : Address, client : &Arc<Provider<Ws>>)  -> eyre::Result<(PoolCreate)> {

    abigen!(
    Factory,
    r#"[
        event PoolCreated(address indexed token0, address indexed token1,uint24 indexed fee,int24 tickSpacing,address pool)
    ]"#,
);

    let factory_contract = Factory::new(factory_address, client.clone());
    let events = factory_contract.event::<PoolCreatedFilter>().from_block(12369621);
    let mut stream = events.stream().await?.take(1);

    let mut pool_create:PoolCreate = PoolCreate {
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

    Ok(pool_create)
}



pub  async fn  subscription_pool_swap (pool_address :Address, client : &Arc<Provider<Ws>> )  -> eyre::Result<()>{
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