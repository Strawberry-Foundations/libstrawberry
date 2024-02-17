use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommunicationError {
    #[error("Packet size too large, expected <=65535, got {0}")]
    PacketTooLarge(usize)
}