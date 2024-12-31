use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcquisitionError {
    #[error("ログファイルの作成に失敗しました: {0}")]
    LogFileCreateError(String),

    #[error("ロガーのロックに失敗しました: {0}")]
    LoggerLockError(String),
}
