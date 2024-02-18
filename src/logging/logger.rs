// src/my_logger.rs

use env_logger::Builder;
use log::LevelFilter;
use std::fs::{self, File};
use std::io::{self, Write};
pub fn init_log(log_dir: &str) -> io::Result<()> {
    // 创建日志文件夹
    fs::create_dir_all(log_dir)?;

    // 配置日志记录
    let log_file = format!("{}/log.log", log_dir); // 设置日志文件路径
    let target = Box::new(File::create(&log_file).expect("Can't create file"));

    Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Pipe(target))
        .init();

    Ok(())
}

pub fn log_info(message: &str) {
    log::info!("{}", message);
}

pub fn log_error(message: &str) {
    log::error!("{}", message);
}
pub fn log_warn(message: &str) {
    log::warn!("{}", message);
}
