use std::time::Duration;

use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::time::timeout;

use eyre::bail;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::stbm::stbchat::error;

pub struct OutgoingPacketStream<AsyncStream: AsyncWriteExt + Unpin> {
    stream: AsyncStream
}

impl<W: AsyncWriteExt + Unpin> OutgoingPacketStream<W> {
    pub const fn wrap(stream: W) -> Self {
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

    pub fn unwrap(self) -> W {
        self.stream
    }

    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.stream
    }
}


pub struct IncomingPacketStream<R: AsyncReadExt + Unpin> {
    stream: R
}

impl<R: AsyncReadExt + Unpin> IncomingPacketStream<R> {
    pub const fn wrap(stream: R) -> Self {
        Self { stream }
    }

    /// # Errors
    /// - Will error ...
    pub async fn read<P: DeserializeOwned>(&mut self) -> eyre::Result<P> {
        let len = self.stream.read_u16().await?;
        let mut buffer = vec![0; len as usize];
        timeout(Duration::from_millis(50), self.stream.read_exact(&mut buffer)).await??;
        Ok(rmp_serde::from_read(buffer.as_slice())?)
    }

    pub fn unwrap(self) -> R {
        self.stream
    }
}