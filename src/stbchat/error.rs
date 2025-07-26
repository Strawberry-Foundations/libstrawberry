use thiserror::Error;

/// Error in case if the package size is too large
#[derive(Error, Debug)]
pub enum CommunicationError {
    #[error("Packet size too large, expected <=65535, got {0}")]
    PacketTooLarge(usize),
}
