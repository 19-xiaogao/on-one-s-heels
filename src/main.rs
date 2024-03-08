use ethers::types::Address;
use eyre::Ok;
use tokio::task;
mod config;
mod logging;
mod migrator;
mod models;
mod subscription;

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
        .unwrap_or_else(|err| {
            logging::log_error(&err.to_string());
            panic!("create client error:{}", err);
        });
    //  let uniswap_pool_address: Address = UNISWAP_POOL_ADDRESS.parse().unwrap();
    let uniswap_factory_address: Address = config
        .block_chain
        .uniswap_factory_v3_address
        .parse()
        .unwrap();

    let nonfungible_position_manager_address: Address = config
        .block_chain
        .nonfungible_position_manager_address
        .parse()
        .unwrap();
    let cloned_client = client.clone();
    let cloned_db = db.clone();
    let cloned_factory_address = uniswap_factory_address.clone();
    task::spawn(async move {
        subscription::subscription_nonfungible_position_manager_mint(
            nonfungible_position_manager_address,
            cloned_factory_address,
            &cloned_client,
            &cloned_db,
        )
        .await;
    });

    /****************************************************************************************************************/
    let cloned2_client = client.clone();
    let pool_address: Address = config.block_chain.pool_address_v3.parse().unwrap();
    let start_block_number = config.block_chain.pool_address_start_block;
    task::spawn(async move {
        subscription::subscription_pool(pool_address, start_block_number, &cloned2_client).await;
    });
    /****************************************************************************************************************/
    subscription::subscription_factory_pool_create(uniswap_factory_address, &client, &db).await;

    Ok(())
}
