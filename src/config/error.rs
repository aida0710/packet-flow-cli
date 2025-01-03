use chrono::NaiveDateTime;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("環境変数ファイルの読み込みに失敗しました: {0}")]
    EnvFileReadError(String),

    #[error("環境変数の取得に失敗しました: {0}")]
    EnvVarError(String),

    #[error("環境変数の解析に失敗しました: {0}")]
    EnvVarParseError(String),

    #[error("終了日時は開始日時より後である必要があります: {0} > {1}")]
    InvalidDateOrder(NaiveDateTime, NaiveDateTime),

    #[error("入力は14桁である必要がありますが、受け取った長さは {0} 桁です。")]
    InvalidInputLength(String),

    #[error("入出力エラー: {0}")]
    IoError(String),

    #[error("日時のパースエラー: {0}")]
    ParseError(String),
}
