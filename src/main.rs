mod config;
mod database;
mod error;
mod interface;
mod logger;
mod packet;
mod utils;

use crate::config::{AppConfig, DateTimeInput};
use crate::database::Database;
use crate::error::InitProcessError;
use crate::interface::select_interface;
use crate::logger::setup_logger::setup_logger;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use log::{error, info};
use std::io;
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> Result<(), InitProcessError> {
    // 設定の読み込み
    let config: AppConfig =
        AppConfig::new().map_err(|e| InitProcessError::ConfigurationError(e.to_string()))?;

    // ロガーのセットアップ
    setup_logger(config.logger_config).map_err(|e| InitProcessError::LoggerError(e.to_string()))?;

    info!("loggerが正常にセットアップされました");
    idps_log!("idps logの表示が有効になっています");

    info!("Node IDは{}に指定されています", config.node_id);

    // データベース接続
    Database::connect(
        &config.database.host,
        config.database.port,
        &config.database.user,
        &config.database.password,
        &config.database.database,
    )
    .await
    .map_err(|e| InitProcessError::DatabaseConnectionError(e.to_string()))?;

    info!(
        "データベースに接続できました: address:{}, port:{}",
        config.database.host, config.database.port
    );

    // ネットワークインターフェースの選択
    let interface = select_interface(
        config.network.docker_mode,
        &config.network.docker_interface_name,
    )
    .map_err(|e| InitProcessError::InterfaceSelectionError(e.to_string()))?;
    info!("デバイスの選択に成功しました: {}", interface.name);

    let datetime_input = DateTimeInput::new().map_err(|e| InitProcessError::InvalidDateInput(e.to_string()))?;
    info!("開始時刻: {}", datetime_input.start_datetime());
    info!("終了時刻: {}", datetime_input.end_datetime());

    Ok(())
}
