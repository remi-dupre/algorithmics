//! https://protohackers.com/problem/4
//!
//! It's your first day on the job. Your predecessor, Ken, left in mysterious circumstances, but
//! not before coming up with a protocol for the new key-value database. You have some doubts about
//! Ken's motivations, but there's no time for questions! Let's implement his protocol.
//!
//!Ken's strange database is a key-value store accessed over UDP. Since UDP does not provide
//!retransmission of dropped packets, and neither does Ken's protocol, clients have to be careful
//!not to send requests too fast, and have to accept that some requests or responses may be
//!dropped.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tokio::net::UdpSocket;
use tracing::{debug, error, info, trace, warn};

use protohackers::init_logs;

const UDP_BUF_SIZE: usize = 1000;

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
    let sock = Arc::new(UdpSocket::bind(format!("{ip}:{port}")).await?);
    let mut buf = [0; UDP_BUF_SIZE];
    let mut db = HashMap::new();

    loop {
        let (size, sender) = sock.recv_from(&mut buf).await?;
        trace!(size = size, "Received bytes");

        let inst = match std::str::from_utf8(&buf[..size]) {
            Ok(x) => x,
            Err(_) => {
                error!("Ignoring query with invalid UTF-8");
                continue;
            }
        };

        debug!(inst = inst, "Received instruction");

        if let Some((key, val)) = inst.split_once('=') {
            info!(key = key, val = val, "Set");

            if key == "version" {
                warn!("Attempt to modify version ignored");
                continue;
            }

            db.insert(key.to_string(), val.to_string());
        } else {
            let key = inst;

            let val = {
                if key == "version" {
                    Some("Dumb Key-Value Store 1.0")
                } else {
                    db.get(key).map(|x| x.as_str())
                }
            };

            info!(key = key, val = val, "Get");

            if let Some(val) = val {
                let sock = sock.clone();
                let resp = format!("{key}={val}");

                tokio::spawn(async move {
                    sock.send_to(resp.as_bytes(), sender)
                        .await
                        .expect("Could not send response")
                });
            }
        }
    }
}
