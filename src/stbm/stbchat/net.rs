use std::io::Cursor;
use std::time::Duration;

use tokio::io::{AsyncWriteExt};

use eyre::bail;
use serde::Serialize;

use crate::stbm::stbchat::error;

pub struct OutgoingPacketStream<AsyncStream: AsyncWriteExt + Unpin> {
    stream: AsyncStream
}

impl<AsyncStream: AsyncWriteExt + Unpin> OutgoingPacketStream<AsyncStream> {
    pub const fn wrap(stream: AsyncStream) -> Self {
        Self { stream }
    }

    /// # Errors
    /// - Will return `Err` if packet size is too large
    ///
    /// # Panics
    /// - Will panic if something gone wrong

    pub async fn write<P: Serialize>(&mut self, packet: P) -> eyre::Result<()> {
        let bytes = rmp_serde::to_vec(&packet)?;
        let len  = bytes.len();

        let Ok(len) = u16::try_from(len) else {
            bail!(error::CommunicationError::PacketTooLarge(len))
        };

        let mut packet = vec![];

        packet.write_u16(len).await.unwrap();
        packet.extend(bytes);

        self.stream.write_all(&packet).await?;

        Ok(())
    }

    pub fn unwrap(self) -> AsyncStream {
        self.stream
    }

    pub fn inner_mut(&mut self) -> &mut AsyncStream {
        &mut self.stream
    }
}