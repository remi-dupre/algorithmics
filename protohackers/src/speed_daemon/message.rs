use std::sync::Arc;

use anyhow::anyhow;
use futures::stream::{Stream, StreamExt};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::Mutex;
use tracing::debug;

use super::error::{ContextExt, Result};
use super::util::{deserialize_roads, deserialize_str, serialize_str};

const MSG_TYPE_ERROR: u8 = 0x10;
const MSG_TYPE_PLATE: u8 = 0x20;
const MSG_TYPE_TICKET: u8 = 0x21;
const MSG_TYPE_WANTHEARTBEAT: u8 = 0x40;
const MSG_TYPE_HEARTBEAT: u8 = 0x41;
const MSG_TYPE_IAMCAMERA: u8 = 0x80;
const MSG_TYPE_IAMDISPATCHER: u8 = 0x81;

pub async fn warn_client(msg: impl Into<String>, write: impl AsyncWrite + Unpin) -> Result<()> {
    let msg = ErrorMessage { msg: msg.into() };

    msg.serialize(write)
        .await
        .context("could not send error message")?;

    Ok(())
}

pub fn message_stream(
    from: impl AsyncRead + Unpin,
) -> impl Stream<Item = Result<ClientMessage>> + Unpin {
    // This mutex will never block as long as iterations of the streams are not called in
    // concurrency, but it is much easier to ensure it at compile time.
    //
    // We are using tokio's Mutex beside being likely to be less efficient because its guard is
    // Send.
    let from = Arc::new(Mutex::new(from));

    // As messages deserializer is not cancellable, we ensure to never drop its future and use a
    // factory to build next future each time it completes.
    let get_message_fut = move || {
        let from = from.clone();

        #[allow(clippy::await_holding_lock)]
        async move {
            let mut guard = from
                .try_lock()
                .expect("message_stream iterations should not be parallelized");

            ClientMessage::deserialize(&mut *guard).await.transpose()
        }
    };

    futures::stream::repeat_with(get_message_fut)
        .buffered(1)
        .take_while(|x| futures::future::ready(x.is_some()))
        .filter_map(futures::future::ready)
}

#[derive(Debug)]
pub enum ClientMessage {
    Plate {
        plate: String,
        timestamp: u32,
    },
    WantHeartbeat {
        interval: u32, // deciseconds
    },
    IAmCamera {
        road: u16,
        mile: u16,
        limit: u16, // miles per hour
    },
    IAmDispatcher {
        roads: Vec<u16>,
    },
}

impl ClientMessage {
    async fn deserialize(mut from: impl AsyncRead + Unpin) -> Result<Option<Self>> {
        let msg_type = match from.read_u8().await {
            Ok(x) => u8::from_be(x),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    return Ok(None);
                } else {
                    return Err(err.into());
                }
            }
        };

        let msg = match msg_type {
            MSG_TYPE_PLATE => {
                let plate = deserialize_str(&mut from).await?;
                let timestamp = from.read_u32().await?;
                Self::Plate { plate, timestamp }
            }
            MSG_TYPE_WANTHEARTBEAT => {
                let interval = from.read_u32().await?;
                Self::WantHeartbeat { interval }
            }
            MSG_TYPE_IAMCAMERA => {
                let road = from.read_u16().await?;
                let mile = from.read_u16().await?;
                let limit = from.read_u16().await?;
                Self::IAmCamera { road, mile, limit }
            }
            MSG_TYPE_IAMDISPATCHER => {
                let roads = deserialize_roads(&mut from).await?;
                Self::IAmDispatcher { roads }
            }
            _ => return Err(anyhow!("unexpected message from client: {msg_type}").into()),
        };

        debug!("Received msg: {msg:?}");
        Ok(Some(msg))
    }
}

#[derive(Clone, Debug)]
pub struct TicketMessage {
    pub plate: String,
    pub road: u16,
    pub mile1: u16,
    pub timestamp1: u32,
    pub mile2: u16,
    pub timestamp2: u32,
    pub speed: u16,
}

impl TicketMessage {
    pub async fn serialize(self, mut out: impl AsyncWrite + Unpin) -> Result<()> {
        out.write_u8(MSG_TYPE_TICKET.to_be()).await?;
        serialize_str(self.plate, &mut out).await?;
        out.write_u16(self.road).await?;
        out.write_u16(self.mile1).await?;
        out.write_u32(self.timestamp1).await?;
        out.write_u16(self.mile2).await?;
        out.write_u32(self.timestamp2).await?;
        out.write_u16(self.speed).await?;
        Ok(())
    }
}

pub struct ErrorMessage {
    pub(crate) msg: String,
}

impl ErrorMessage {
    pub async fn serialize(self, mut out: impl AsyncWrite + Unpin) -> Result<()> {
        out.write_u8(MSG_TYPE_ERROR.to_be()).await?;
        serialize_str(self.msg, &mut out).await?;
        Ok(())
    }
}

pub struct HeartbeatMessage {}

impl HeartbeatMessage {
    pub async fn serialize(self, mut out: impl AsyncWrite + Unpin) -> Result<()> {
        out.write_u8(MSG_TYPE_HEARTBEAT.to_be()).await?;
        Ok(())
    }
}
