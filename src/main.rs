use std::sync::Arc;
use ethers::prelude::*;
use ethers::utils::{parse_units, ParseUnits};

const WSS_URL: &str = "wss://ethereum.publicnode.com";

abigen!(
    UniswapFactroy,
    r#"[
         event PoolCreated(address indexed token0, address indexed token1,uint24 indexed fee,int24 tickSpacing,address pool)
    ]"#,
);

abigen!(
    UniswapPool,
    r#"[
        event Swap(address indexed sender,address indexed recipient,int256 amount0,int256 amount1,uint160 sqrtPriceX96,uint128 liquidity,int24 tick)
        function balance0() external view returns (uint256)
        function balance1() external view returns (uint256)
    ]"#,
);

const UNISWAP_FACTORY_ADDRESS: &str ="0x1f98431c8ad98523631ae4a59f267346ea31f984";
const UNISWAP_POOL_ADDRESS: &str ="0x331399c614cA67DEe86733E5A2FBA40DbB16827c";

//  需求: 监听 uniswap V3 factory 池子的创建。当第一次流动性的token 大于某值的时候, 买入一笔交易。
#[tokio::main]
async fn main() -> eyre::Result<()> {

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let  client = Arc::new(provider);
    let address: Address = UNISWAP_FACTORY_ADDRESS.parse()?;
    let factory_contract = UniswapFactroy::new(address, client.clone());
    let uniswap_pool_address : Address = UNISWAP_POOL_ADDRESS.parse()?;
    let pool_contract  = UniswapPool::new(uniswap_pool_address, client.clone());

    loop {
        if let Ok(_balance0) = pool_contract.balance_0().call().await {
            println!("sender:{}?",_balance0);
        }

        let events = pool_contract.event::<SwapFilter>().from_block(19117504);
        let mut stream = events.stream().await?.take(1);
        while let Some(Ok(f)) = stream.next().await {
            println!("sender:{}",f.sender);
            println!("recipient:{}",f.recipient);
            println!("amount_0:{}",f.amount_0);
            println!("amount_1::{}",f.amount_1);
            println!("sqrt_price_x96:::{}",f.sqrt_price_x96);
            println!("liquidity::::{}",f.liquidity);
            println!("tick::::{}",f.tick);
            println!("---------------------------------------------------------");
        }
    }
}

