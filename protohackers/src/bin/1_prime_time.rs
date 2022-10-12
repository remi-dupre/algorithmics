use anyhow::Result;
use futures::StreamExt;
use num_bigint::BigInt;
use serde::Deserialize;
use serde_json::value::RawValue;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, trace, warn};

use std::str::FromStr;

use protohackers::{init_logs, split_at_bytes};

#[derive(Deserialize)]
struct Request<'a> {
    method: &'a str,
    number: Box<RawValue>,
}

enum Number {
    Int(i64),
    Big(BigInt),
    Float(f64),
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = i64::from_str(s) {
            return Ok(Self::Int(x));
        }

        if let Ok(x) = BigInt::from_str(s) {
            return Ok(Self::Big(x));
        }

        if let Ok(x) = f64::from_str(s) {
            return Ok(Self::Float(x));
        }

        Err(())
    }
}

impl Number {
    fn is_prime(&self) -> bool {
        match self {
            Number::Int(x) => *x >= 2 && (2..).take_while(|q| q * q <= *x).all(|q| x % q != 0),
            Number::Float(x) => x.floor() == *x && Self::Int(*x as _).is_prime(),
            Number::Big(x) => {
                let b_0 = BigInt::from(0);
                let b_1 = BigInt::from(1);
                let b_2 = BigInt::from(2);

                if x < &b_2 {
                    return false;
                }

                let sqrt = x.sqrt();
                let mut q = b_2;

                while q <= sqrt {
                    if x % &q == b_0 {
                        return false;
                    }

                    q += &b_1;
                }

                true
            }
        }
    }
}

#[tracing::instrument(skip(socket))]
async fn client(id: u64, socket: TcpStream) -> Result<()> {
    info!("Connected");
    let (reader, mut writer) = socket.into_split();
    let mut lines = Box::pin(split_at_bytes(&[b'\n', 10], reader));

    while let Some(line) = lines.next().await {
        let line = line?;

        trace!(
            line = String::from_utf8_lossy(&line).to_string(),
            "Received",
        );

        let req: Request = match serde_json::from_slice(&line) {
            Ok(x) => x,
            Err(err) => {
                warn!("Invalid JSON: {err}");
                writer.write_all(b"Malformed request").await?;
                break;
            }
        };

        let number = match Number::from_str(req.number.get()) {
            Ok(x) => x,
            Err(_) => {
                warn!("Invalid number: {}", req.number);
                writer.write_all(b"Invalid number").await?;
                break;
            }
        };

        info!("Received request: {} {}", req.method, req.number);

        if req.method == "isPrime" {
            let result = number.is_prime();
            info!("Answer: {result}");

            writer
                .write_all(format!(r#"{{"method":"isPrime","prime":{result}}}"#).as_bytes())
                .await?;

            writer.write_u8(b'\n').await?;
        } else {
            writer.write_all(b"Unknown method").await?;
            break;
        }
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
