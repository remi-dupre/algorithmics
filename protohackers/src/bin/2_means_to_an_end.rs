//! https://protohackers.com/problem/2
//!
//! Your friendly neighbourhood investment bank is having trouble analysing historical price data.
//! They need you to build a TCP server that will let clients insert and query timestamped prices.

use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, trace, warn};

use protohackers::init_logs;

const BUF_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
enum Op {
    Insert { timestamp: i32, price: i32 },
    Query { mintime: i32, maxtime: i32 },
}

impl Op {
    fn decode(bytes: [u8; 9]) -> Result<Self> {
        let x = i32::from_be_bytes(bytes[1..5].try_into().unwrap());
        let y = i32::from_be_bytes(bytes[5..9].try_into().unwrap());

        match bytes[0] {
            b'I' => Ok(Self::Insert {
                timestamp: x,
                price: y,
            }),
            b'Q' => Ok(Self::Query {
                mintime: x,
                maxtime: y,
            }),
            op => Err(anyhow!("Invalid operation {op}")),
        }
    }
}

#[tracing::instrument(skip(socket))]
async fn client(id: u64, socket: TcpStream) -> Result<()> {
    info!("Connected");
    let mut db: BTreeMap<i32, i32> = BTreeMap::new();
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::with_capacity(BUF_SIZE, reader);
    let mut buffer = [0; 9];

    while reader.read_exact(&mut buffer).await.is_ok() {
        trace!("Received buffer: {buffer:02x?}");

        let op = match Op::decode(buffer) {
            Ok(x) => x,
            Err(err) => {
                warn!("Couldn't decode operation: {err:?}");
                continue;
            }
        };

        info!("Received operation: {op:?}");

        match op {
            Op::Insert { timestamp, price } => {
                db.insert(timestamp, price);
            }
            Op::Query { mintime, maxtime } => {
                let prices: Vec<_> = db
                    .iter()
                    .skip_while(|&(t, _)| *t < mintime)
                    .take_while(|&(t, _)| *t <= maxtime)
                    .map(|(_, &p)| p)
                    .collect();

                let count = prices.len() as i64;
                let sum: i64 = prices.into_iter().map(i64::from).sum();
                let mean = if count > 0 { (sum / count) as i32 } else { 0 };

                info!(mean = mean, "Answering");
                writer.write_all(&mean.to_be_bytes()).await?;
            }
        };
    }

    info!("Disconnected");
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
        tokio::spawn(client(client_count, socket));
        client_count += 1;
    }
}
