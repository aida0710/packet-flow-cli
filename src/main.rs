mod config;
mod database;
mod error;
mod interface;
mod logger;
mod packet;
mod utils;

use crate::config::AppConfig;
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

    // 開始日時の入力
    let start_datetime = get_datetime_input("開始日時を入力してください")?;
    println!("入力された開始日時: {}", start_datetime);

    // 終了日時の入力
    let end_datetime = get_datetime_input("終了日時を入力してください")?;
    println!("入力された終了日時: {}", end_datetime);

    // 日時の妥当性チェック
    if end_datetime <= start_datetime {
        return Err("終了日時は開始日時より後である必要があります".into());
    }

    Ok(())
}

fn get_datetime_input(prompt: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
    print!("{} (形式: YYYYMMDDHHMMSS): ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // 入力された文字列をパースして日時に変換
    if input.len() != 14 {
        return Err("入力は14桁である必要があります (YYYYMMDDHHMMSS)".into());
    }

    let year = &input[0..4].parse::<i32>()?;
    let month = &input[4..6].parse::<u32>()?;
    let day = &input[6..8].parse::<u32>()?;
    let hour = &input[8..10].parse::<u32>()?;
    let minute = &input[10..12].parse::<u32>()?;
    let second = &input[12..14].parse::<u32>()?;

    let datetime = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(*year, *month, *day).ok_or("無効な日付です")?,
        NaiveTime::from_hms_opt(*hour, *minute, *second).ok_or("無効な時刻です")?,
    );

    Ok(datetime)
}
