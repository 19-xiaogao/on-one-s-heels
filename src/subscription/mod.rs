use std::sync::Arc;
use ethers::contract::abigen;
use ethers::prelude::{Address, Provider, ProviderError, Ws};
use crate::SwapFilter;

mod client;

struct PoolCreate {
    token0 : Address,
    token1 : Address,
    fee : u16,
    tick_spacing: i16,
    pool_address : Address
}

// 订阅池子创建
pub async fn subscription_factory_pool_create(factory_address : String, client : &Arc<Provider<Ws>>) -> Result<PoolCreate,ProviderError> {

    abigen!(
    Factory,
    r#"[
        event PoolCreated(address indexed token0, address indexed token1,uint24 indexed fee,int24 tickSpacing,address pool)
    ]"#,
);

    let factoryContarct = Factory::new(factory_address, client.clone());
    // from block  是否可以查询
    let events = factoryContarct.event::<PoolCreatedFilter>().from_block(19117504);
    let mut stream = events.stream().await?.take(1);

    // while let Some(Ok(val)) = stream.next().await {
    //     Ok()
    //     // println!("sender:{}", f.sender);
    //     // println!("recipient:{}", f.recipient);
    //     // println!("amount_0:{}", f.amount_0);
    //     // println!("amount_1::{}", f.amount_1);
    //     // println!("sqrt_price_x96:::{}", f.sqrt_price_x96);
    //     // println!("liquidity::::{}", f.liquidity);
    //     // println!("tick::::{}", f.tick);
    //     // println!("---------------------------------------------------------");
    // }
}
