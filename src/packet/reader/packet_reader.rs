use crate::packet::reader::error::PacketReaderError;
use crate::packet::reader::packet_sender::PacketSender;
use crate::packet::repository::PacketRepository;
use chrono::{DateTime, Utc};
use log::{error, info};
use pnet::datalink::NetworkInterface;

pub struct PacketReader;

impl PacketReader {
    pub async fn replay_packets(interface: NetworkInterface, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<(), PacketReaderError> {
        info!("パケット再生を開始します");
        info!("期間: {} から {}", start_time, end_time);

        match PacketRepository::get_packets_in_timerange(start_time, end_time).await {
            Ok(packets) => {
                info!("{}個のパケットを取得しました", packets.len());
                PacketSender::send_packets_with_timing(&interface, packets).await?;
                info!("パケット再生が完了しました");
                Ok(())
            },
            Err(e) => {
                error!("パケットの取得に失敗しました: {:?}", e);
                Err(PacketReaderError::ConfigurationError(e.to_string()))
            },
        }
    }
}
