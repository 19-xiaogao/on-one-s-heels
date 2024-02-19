use ethers::{abi::AbiEncode, prelude::*, utils::hex::ToHexExt};
mod config;
mod logging;
mod migrator;
mod models;
mod subscription;
use chrono::{Local, NaiveDateTime};
use log::kv::ToValue;

//  需求: 监听 uniswap V3 factory 池子的创建。当第一次流动性的token 大于某值的时候, 买入一笔交易。
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config =
        config::read_config().unwrap_or_else(|err| panic!("read config file error:{}", err));
    //  从配置中读取日志写入目录。
    logging::init_log(&config.log.log_dir).unwrap_or_else(|err| println!("init log err :{}", err));

    // connect data base
    let db = models::connect_to_database(config.database)
        .await
        .unwrap_or_else(|err| {
            logging::log_error(&err.to_string());
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        });
    // sync table col
    migrator::run(&db).await.unwrap_or_else(|err| {
        logging::log_warn(&err.to_string());
        println!("init database err :{}", err)
    });

    let client = subscription::create_client(&config.block_chain.ws_url)
        .await
        .unwrap_or_else(|err| panic!("create client error:{}", err));
    let uniswap_factory_address: Address = config
        .block_chain
        .uniswap_factory_v3_address
        .parse()
        .unwrap();
    loop {
        let pool_create =
            subscription::subscription_factory_pool_create(uniswap_factory_address, &client)
                .await
                .expect("todo:err");
        println!("pool create :{:?}", pool_create);
        let create_time = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0).unwrap();
        models::insert_pool(
            &db,
            models::Model {
                id: Default::default(),
                token0: pool_create.token0.encode_hex_with_prefix(),
                token1: pool_create.token1.encode_hex_with_prefix(),
                pool_address: pool_create.pool_address.encode_hex_with_prefix(),
                fee: pool_create.fee as i32,
                tick_spacing: pool_create.tick_spacing as i32,
                create_time,
            },
        )
        .await?;
    }
}
