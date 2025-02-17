//! https://protohackers.com/problem/5
//!
//! You're escorted to a dark, smoky, basement office. Big Tony sits the other side of a large
//! desk, leaning back in his chair, puffing on a cigar that you can only describe as
//! comedically-oversized. Two of his goons loiter in the doorway. They are tall and wide but not
//! obviously very bright, which only makes them all the more intimidating. Tony flashes a menacing
//! grin, revealing an unusual number of gold-plated teeth, and makes you an offer you can't
//! refuse: he wants you to write a malicious proxy server for Budget Chat.
//!
//! For each client that connects to your proxy server, you'll make a corresponding outward
//! connection to the upstream server. When the client sends a message to your proxy, you'll pass
//! it on upstream. When the upstream server sends a message to your proxy, you'll pass it on
//! downstream. Remember that messages in Budget Chat are delimited by newline characters ('\n', or
//! ASCII 10).
//!
//! Most messages are passed back and forth without modification, so that the client believes it is
//! talking directly to the upstream server, except that you will be rewriting Boguscoin addresses,
//! in both directions, so that all payments go to Tony.

#![feature(byte_slice_trim_ascii)]
#![feature(iter_intersperse)]

use std::borrow::Cow;

use anyhow::{Context, Result};
use futures::StreamExt;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, info_span, Instrument};

use protohackers::{init_logs, split_at_bytes};

const REMOTE_SERVER: &str = "chat.protohackers.com:16963";
const BOGUSCOIN_TONY: &[u8] = b"7YWHMfk9JZe0LM0g1ZauHuiSxhI";

/// Check if the given slice is a valid boguscoin address.
fn is_boguscoin(word: &[u8]) -> bool {
    word.first() == Some(&b'7')
        && (26..37).contains(&word.len())
        && word.iter().all(|b| b.is_ascii_alphanumeric())
}

/// Replace boguscoin addresses with Tony's address.
fn impersonate_boguscoins(msg: &[u8]) -> Cow<[u8]> {
    let words = msg.trim_ascii_end().split(|b| *b == b' ');

    if !words.clone().any(is_boguscoin) {
        Cow::Borrowed(msg)
    } else {
        let msg = words
            .map(|word| {
                if is_boguscoin(word) {
                    BOGUSCOIN_TONY
                } else {
                    word
                }
            })
            .intersperse(b" ")
            .chain([b"\n".as_slice()])
            .flatten()
            .copied()
            .collect();

        Cow::Owned(msg)
    }
}

/// Impersonate then sends a message. Return false if the proxy should be stopped.
async fn transmit(msg: Option<Vec<u8>>, mut dest: impl AsyncWrite + Unpin) -> Result<bool> {
    if let Some(msg) = msg {
        debug!(msg = String::from_utf8_lossy(&msg).as_ref(), "Received");
        let msg = impersonate_boguscoins(&msg);

        info!(msg = String::from_utf8_lossy(&msg).as_ref(), "Sending");
        dest.write_all(&msg).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tracing::instrument(skip(socket))]
async fn proxy(id: u64, socket: TcpStream) -> Result<()> {
    info!("Starting");

    let (client_reader, mut client_writer) = socket.into_split();
    let (server_reader, mut server_writer) = TcpStream::connect(REMOTE_SERVER)
        .await
        .context("Could not connect to server")?
        .into_split();

    let mut client_lines = Box::pin(split_at_bytes(&[b'\n'], client_reader).fuse());
    let mut server_lines = Box::pin(split_at_bytes(&[b'\n'], server_reader).fuse());

    loop {
        futures::select_biased! {
            msg = client_lines.next() => {
                if !transmit(msg.transpose()?, &mut server_writer)
                    .instrument(info_span!("client->server"))
                    .await?
                {
                    break;
                }
            },
            msg = server_lines.next() => {
                if !transmit(msg.transpose()?, &mut client_writer)
                    .instrument(info_span!("server->client"))
                    .await?
                {
                    break;
                }
            },
        }
    }

    info!("Closing");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    init_logs();

    let ip = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "8080".to_string());

    info!("Running on port {ip}:{port}");
    let listener = TcpListener::bind(format!(":::{port}")).await?;
    let mut client_count = 0;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(proxy(client_count, socket));
        client_count += 1;
    }
}
