use std::sync::Arc;

use anyhow::anyhow;
use futures::future::FutureExt;
use futures::stream::StreamExt;
use futures::Stream;
use tokio::net::{tcp, TcpStream};
use tokio::select;
use tracing::{error, info, warn};

use super::error::{ContextExt, Error, Result};
use super::heartbeat::Hearbeat;
use super::message::warn_client;
use super::message::{message_stream, ClientMessage, HeartbeatMessage};
use super::state::{SeenPlate, State};

pub async fn send_heartbeat(tcp_write: &mut tcp::OwnedWriteHalf) -> Result<()> {
    info!("Heartbeat");
    let msg = HeartbeatMessage {};

    msg.serialize(tcp_write)
        .await
        .context("could not send heartbeat")?;

    Ok(())
}

#[tracing::instrument(skip(state, tcp))]
pub async fn client(id: usize, state: Arc<State>, tcp: TcpStream) {
    info!("Connected");
    let (mut tcp_read, mut tcp_write) = tcp.into_split();
    let res = client_impl(state, &mut tcp_read, &mut tcp_write).await;

    match res {
        Ok(()) => info!("Disconnected"),
        Err(err) => match err {
            Error::Server(err) => {
                error!("Server error: {err:?}");
            }
            Error::Client(err) => {
                warn!("Client error: {err}");

                warn_client(format!("Invalid behavior: {err}"), &mut tcp_write)
                    .await
                    .map_err(|err| warn!("Could not send warning to client: {err}"))
                    .ok();
            }
        },
    };
}

async fn client_impl(
    state: Arc<State>,
    tcp_read: &mut tcp::OwnedReadHalf,
    tcp_write: &mut tcp::OwnedWriteHalf,
) -> Result<()> {
    let mut messages = message_stream(tcp_read);
    let mut heartbeat = Hearbeat::default();

    loop {
        select! {
            _ = heartbeat.tick() => send_heartbeat(tcp_write).await?,
            message = messages.next() => {
                let message = match message {
                    None => break,
                    Some(x) => x.map_err(Error::into_client)?,
                };

                match message {
                    ClientMessage::WantHeartbeat { interval } => heartbeat.set(interval)?,
                    ClientMessage::IAmCamera { road, mile, limit } => {
                        client_camera(state, messages, tcp_write, heartbeat, road, mile, limit)
                            .await?;

                        break;
                    }
                    ClientMessage::IAmDispatcher { roads } => {
                        client_dispatcher(state, messages, tcp_write, heartbeat, roads).await?;
                        break;
                    }
                    msg => {
                        return Err(Error::Client(
                            anyhow!("Unsupported message for undeclared client {msg:?}")
                        ));
                    }
                };
            }
        }
    }

    Ok(())
}

#[tracing::instrument(name = "camera", skip(state, messages, tcp_write, heartbeat))]
async fn client_camera(
    state: Arc<State>,
    mut messages: impl Stream<Item = Result<ClientMessage>> + Unpin,
    tcp_write: &mut tcp::OwnedWriteHalf,
    mut heartbeat: Hearbeat,
    road: u16,
    mile: u16,
    limit: u16,
) -> Result<()> {
    info!("Initializing");
    state.register_road(road, limit).await;
    let plate_sender = state.get_plate_sender(road).await;

    loop {
        select! {
            _ = heartbeat.tick() => send_heartbeat(tcp_write).await?,
            message = messages.next() => {
                let message = match message {
                    None => break,
                    Some(x) => x.map_err(Error::into_client)?,
                };

                match message {
                    ClientMessage::WantHeartbeat { interval } => heartbeat.set(interval)?,
                    ClientMessage::Plate { plate, timestamp } => {
                        plate_sender
                            .send(SeenPlate {
                                plate,
                                timestamp,
                                road,
                                mile,
                            })
                            .await
                            .expect("plate channel closed unexpectedly");
                    }
                    msg => {
                        return Err(Error::Client(
                            anyhow!("Unsupported message for camera {msg:?}")
                        ));
                    }
                };
            }
        }
    }

    Ok(())
}

#[tracing::instrument(name = "dispatcher", skip(state, messages, tcp_write, heartbeat))]
async fn client_dispatcher(
    state: Arc<State>,
    mut messages: impl Stream<Item = Result<ClientMessage>> + Unpin,
    mut tcp_write: &mut tcp::OwnedWriteHalf,
    mut heartbeat: Hearbeat,
    roads: Vec<u16>,
) -> Result<()> {
    info!("Initializing");

    // Merge ticket streams for all subscribed roads
    let mut tickets_stream = {
        let ticket_receivers: Vec<_> = futures::stream::iter(roads)
            .filter_map(|road| state.get_ticket_receiver(road).map(Some))
            .collect()
            .await;

        futures::stream::select(
            futures::stream::select_all(ticket_receivers),
            futures::stream::pending(), // avoid end of stream when there is no receivers
        )
    };

    loop {
        select! {
            _ = heartbeat.tick() => send_heartbeat(tcp_write).await?,
            message = messages.next() => {
                let message = match message {
                    None => break,
                    Some(x) => x.map_err(Error::into_client)?,
                };

                match message {
                    ClientMessage::WantHeartbeat { interval } => heartbeat.set(interval)?,
                    msg => {
                        return Err(Error::Client(
                            anyhow!("Unsupported message for dispatcher {msg:?}")
                        ));
                    }
                };
            },
            ticket = tickets_stream.next() => {
                // It is assured that channel writers will never be dropped. In the case were no
                // road was specified in the input we will wait for a pending stream anyway.
                let ticket = ticket.expect("ticket stream stop unexpectedly");
                info!("Sending {ticket:?}");
                ticket.serialize(&mut tcp_write).await?;
            }
        }
    }

    Ok(())
}
