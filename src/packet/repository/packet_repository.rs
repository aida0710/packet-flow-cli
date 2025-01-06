use crate::database::{Database, DatabaseError, ExecuteQuery};
use chrono::{DateTime, Utc};

pub struct PacketRepository;

impl PacketRepository {
    pub async fn get_packets_in_timerange(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<Vec<(DateTime<Utc>, Vec<u8>)>, DatabaseError> {
        let db = Database::get_database();
        let query = "
            SELECT timestamp, raw_packet
            FROM packets
            WHERE timestamp >= $1 AND timestamp <= $2
            ORDER BY timestamp ASC";

        let rows = db.query(query, &[&start_time, &end_time]).await?;
        Ok(rows.into_iter().map(|row| (row.get("timestamp"), row.get("raw_packet"))).collect())
    }

    pub async fn get_packet_time_range() -> Result<(DateTime<Utc>, DateTime<Utc>), DatabaseError> {
        let db = Database::get_database();
        let query = "
            SELECT
                MIN(timestamp) as min_time,
                MAX(timestamp) as max_time
            FROM packets";

        let rows = db.query(query, &[]).await?;
        if let Some(row) = rows.first() {
            let min_time: DateTime<Utc> = row.get("min_time");
            let max_time: DateTime<Utc> = row.get("max_time");
            Ok((min_time, max_time))
        } else {
            Err(DatabaseError::QueryExecutionError("パケットが存在しません".to_string()))
        }
    }
}
