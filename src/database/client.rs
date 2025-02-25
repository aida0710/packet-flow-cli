use crate::database::error::DatabaseError;
use crate::database::pool::DatabasePool;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio_postgres::{Row, Statement};

#[async_trait]
pub trait ExecuteQuery {
    async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, DatabaseError>;
}

pub struct Database {
    prepared_statements: HashMap<String, Statement>,
}

impl Database {
    pub async fn connect(host: &str, port: u16, user: &str, password: &str, database: &str) -> Result<(), DatabaseError> {
        DatabasePool::initialize(host, port, user, password, database).await
    }

    pub fn get_database() -> &'static Self {
        // DbPoolの存在を確認
        let _ = DatabasePool::get_pool();
        // Databaseはステートレスなので、staticなインスタンスを返す
        static DATABASE: std::sync::OnceLock<Database> = std::sync::OnceLock::new();
        DATABASE.get_or_init(|| Database {
            prepared_statements: HashMap::new(),
        })
    }
}

#[async_trait]
impl ExecuteQuery for Database {
    async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, DatabaseError> {
        let pool = DatabasePool::get_pool().map_err(|e| DatabaseError::PoolRetrievalError(e.to_string()))?;
        let client = pool.inner().get().await.map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // プリペアドステートメントのキャッシュを試みる
        let stmt = if let Some(stmt) = self.prepared_statements.get(query) {
            stmt.clone()
        } else {
            client.prepare(query).await.map_err(|e| DatabaseError::QueryPreparationError(e.to_string()))?
        };

        let rows = client.query(&stmt, params).await.map_err(|e| DatabaseError::QueryExecutionError(e.to_string()))?;
        Ok(rows)
    }
}
