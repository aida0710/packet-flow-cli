use crate::config::error::ConfigError;
use crate::packet::repository::PacketRepository;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::io::{self, Write};

pub struct DateTimeInput {
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
}

impl DateTimeInput {
    pub async fn new() -> Result<Self, ConfigError> {
        // データベース内のパケットの時間範囲を取得
        match PacketRepository::get_packet_time_range().await {
            Ok((min_time, max_time)) => {
                println!("\nデータベース内のパケットの時間範囲:");
                println!("最古のパケット: {}", min_time);
                println!("最新のパケット: {}\n", max_time);
            },
            Err(e) => {
                println!("警告: パケットの時間範囲の取得に失敗しました: {}", e);
            },
        }

        let start_datetime = loop {
            match get_naive_datetime_input("開始日時を入力してください") {
                Ok(naive_dt) => {
                    break DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                },
                Err(e) => {
                    println!("エラー: {}. もう一度入力してください。", e);
                    continue;
                },
            }
        };
        println!("入力された開始日時: {}", start_datetime);

        let end_datetime = loop {
            match get_naive_datetime_input("終了日時を入力してください") {
                Ok(naive_dt) => {
                    let dt = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                    if dt <= start_datetime {
                        println!("エラー: 終了日時は開始日時より後である必要があります。もう一度入力してください。");
                        continue;
                    }
                    break dt;
                },
                Err(e) => {
                    println!("エラー: {}. もう一度入力してください。", e);
                    continue;
                },
            }
        };
        println!("入力された終了日時: {}", end_datetime);

        Ok(Self { start_datetime, end_datetime })
    }

    pub fn start_datetime(&self) -> DateTime<Utc> {
        self.start_datetime
    }

    pub fn end_datetime(&self) -> DateTime<Utc> {
        self.end_datetime
    }
}

fn get_naive_datetime_input(prompt: &str) -> Result<NaiveDateTime, ConfigError> {
    print!("{} (形式: YYYYMMDDHHMMSS): ", prompt);
    io::stdout().flush().map_err(|e| ConfigError::IoError(e.to_string()))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| ConfigError::IoError(e.to_string()))?;
    let input = input.trim();

    // 入力された文字列をパースして日時に変換
    if input.len() != 14 {
        return Err(ConfigError::InvalidInputLength(input.len().to_string()));
    }

    let year = input[0..4].parse::<i32>().map_err(|e| ConfigError::ParseError(format!("無効な年: {}", e)))?;
    let month = input[4..6].parse::<u32>().map_err(|e| ConfigError::ParseError(format!("無効な月: {}", e)))?;
    let day = input[6..8].parse::<u32>().map_err(|e| ConfigError::ParseError(format!("無効な日: {}", e)))?;
    let hour = input[8..10].parse::<u32>().map_err(|e| ConfigError::ParseError(format!("無効な時: {}", e)))?;
    let minute = input[10..12].parse::<u32>().map_err(|e| ConfigError::ParseError(format!("無効な分: {}", e)))?;
    let second = input[12..14].parse::<u32>().map_err(|e| ConfigError::ParseError(format!("無効な秒: {}", e)))?;

    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ConfigError::ParseError("無効な日付です".to_string()))?;
    let time = NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(|| ConfigError::ParseError("無効な時刻です".to_string()))?;

    Ok(NaiveDateTime::new(date, time))
}
