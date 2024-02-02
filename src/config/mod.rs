use config::{Config, ConfigError, File};
#[derive(Debug)]
pub struct BlockChain {
    pub chain_id: usize,
    pub ws_url: String,
    pub uniswap_factory_v3_address: String,
    pub uniswap_router_v3_address: String,
    pub usdt_address: String,
    pub usdc_address: String,
    pub weth_address: String,
    pub gas_price: usize,
}
pub fn read_config() -> Result<BlockChain, ConfigError> {
    let mut config = Config::new();
    let mut file_path = String::new();
    file_path.push_str("config/config.toml");

    config.merge(File::with_name(file_path.as_str()))?;

    let chain_id = config.get_int("ethereum.chain_id")?;
    let ws_url = config.get_str("ethereum.ws_url")?;
    let uniswap_factory_v3_address = config.get_str("ethereum.uniswap_factory_v3_address")?;
    let uniswap_router_v3_address = config.get_str("ethereum.uniswap_router_v3_address")?;
    let usdt_address = config.get_str("ethereum.usdt_address")?;
    let usdc_address = config.get_str("ethereum.usdc_address")?;
    let weth_address = config.get_str("ethereum.weth_address")?;
    let gas_price = config.get_int("ethereum.gas_price")?;

    Ok(BlockChain {
        chain_id: chain_id.try_into().unwrap(),
        ws_url,
        uniswap_factory_v3_address,
        uniswap_router_v3_address,
        usdt_address,
        usdc_address,
        weth_address,
        gas_price: gas_price.try_into().unwrap(),
    })
}
