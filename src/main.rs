use ethers::types::Address;

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
    println!("address:{:?}", nonfungible_position_manager_address);
    loop {
        // subscription::subscription_pool_swap(uniswap_pool_address, &client).await.expect("TODO: panic message");
        subscription::subscription_factory_pool_create(uniswap_factory_address, &client, &db).await;
        let mint = subscription::subscription_nonfungible_position_manager_mint(
            nonfungible_position_manager_address,
            &client,
        )
        .await;
        println!("mint:{:?}", mint)
    }
}
