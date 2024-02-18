use std::time::Duration;

use crate::config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};


pub async fn connect_to_database(params: config::Database) -> Result<DatabaseConnection, DbErr> {
    let str = "mysql://".to_owned()
        + &params.username
        + ":"
        + &params.password
        + "@"
        + &params.host
        + "/"
        + &params.database_name;

    let mut opt = ConnectOptions::new(str.as_str());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Ok(Database::connect(str.as_str()).await?)
}
