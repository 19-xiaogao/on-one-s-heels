use ethers::prelude::*;
mod config;
mod subscription;
const UNISWAP_POOL_ADDRESS: &str = "0x331399c614cA67DEe86733E5A2FBA40DbB16827c";

//  需求: 监听 uniswap V3 factory 池子的创建。当第一次流动性的token 大于某值的时候, 买入一笔交易。
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config =
    config::read_config().unwrap_or_else(|err| panic!("read config file error:{}", err));

         let client = subscription::create_client( &config.ws_url).await.unwrap();
         let uniswap_pool_address: Address = UNISWAP_POOL_ADDRESS.parse().unwrap();
            let uniswap_factory_address :Address = config.uniswap_factory_v3_address.parse().unwrap();
    loop {
        // subscription::subscription_pool_swap(uniswap_pool_address, &client).await.expect("TODO: panic message");

        let pool_create = subscription::subscription_factory_pool_create(uniswap_factory_address, &client).await.expect("todo:err");
        println!("pool create :{:?}",pool_create);
    }
}
