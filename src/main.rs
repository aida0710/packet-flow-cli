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
use crate::packet::reader::PacketReader;
use log::info;

#[tokio::main]
async fn main() -> Result<(), InitProcessError> {
    // 設定の読み込み
    let config: AppConfig = AppConfig::new().map_err(|e| InitProcessError::ConfigurationError(e.to_string()))?;

    // ロガーのセットアップ
    setup_logger(config.logger_config).map_err(|e| InitProcessError::LoggerError(e.to_string()))?;

    info!("loggerが正常にセットアップされました");

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

    info!("データベースに接続できました: address:{}, port:{}", config.database.host, config.database.port);

    // ネットワークインターフェースの選択
    let interface = select_interface(config.network.docker_mode, &config.network.docker_interface_name).map_err(|e| InitProcessError::InterfaceSelectionError(e.to_string()))?;

    info!("選択されたインターフェース: {}", interface.name);

    // 時間範囲の入力
    let datetime_input = DateTimeInput::new().await.map_err(|e| InitProcessError::InvalidDateInput(e.to_string()))?;

    info!("開始時刻: {}", datetime_input.start_datetime());
    info!("終了時刻: {}", datetime_input.end_datetime());

    // パケット再生の実行
    PacketReader::replay_packets(interface, datetime_input.start_datetime(), datetime_input.end_datetime())
        .await
        .map_err(|e| InitProcessError::TaskExecutionProcessError(e.to_string()))?;

    Ok(())
}
