use std::env;

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

#[derive(Debug)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database_name: String,
}

#[derive(Debug)]
pub struct Service {
    pub port: i64,
}

#[derive(Debug)]
pub struct Log {
    pub log_dir: String,
}

#[derive(Debug)]
pub struct Configs {
    pub database: Database,
    pub server: Service,
    pub log: Log,
    pub block_chain: BlockChain,
}

// read block_chain test network config
pub fn read_block_chain_config(config: &Config) -> Result<BlockChain, ConfigError> {
    let chain_id = config.get_int("block_chain.chain_id")?;
    let ws_url = config.get_str("block_chain.ws_url")?;
    let uniswap_factory_v3_address = config.get_str("block_chain.uniswap_factory_v3_address")?;
    let uniswap_router_v3_address = config.get_str("block_chain.uniswap_router_v3_address")?;
    let usdt_address = config.get_str("block_chain.usdt_address")?;
    let usdc_address = config.get_str("block_chain.usdc_address")?;
    let weth_address = config.get_str("block_chain.weth_address")?;
    let gas_price = config.get_int("block_chain.gas_price")?;

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

fn read_database_config(config: &Config) -> Result<Database, ConfigError> {
    let username = config.get_str("database.username")?;
    let password = config.get_str("database.password")?;
    let host = config.get_str("database.host")?;
    let database_name = config.get_str("database.database_name")?;

    Ok(Database {
        username: username.to_owned(),
        password: password.to_owned(),
        host: host.to_owned(),
        database_name: database_name.to_owned(),
    })
}

fn read_server_config(config: &Config) -> Result<Service, ConfigError> {
    let port = config.get_int("server.port")?;
    Ok(Service { port })
}

fn red_log_config(config: &Config) -> Result<Log, ConfigError> {
    let log_dir = config.get_str("log.log_dir")?;
    Ok(Log { log_dir })
}

pub fn read_config() -> Result<Configs, ConfigError> {
    // read current program env
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let mut config = Config::new();
    let mut file_path = String::new();
    if environment == "development" {
        file_path.push_str("config/config.env.toml");
    } else {
        file_path.push_str("config/config.prod.toml");
    }
    config.merge(File::with_name(file_path.as_str()))?;
    let database = read_database_config(&config)?;
    let server = read_server_config(&config)?;
    let log = red_log_config(&config)?;
    let block_chain = read_block_chain_config(&config)?;
    Ok(Configs {
        database,
        server,
        log,
        block_chain,
    })
}
