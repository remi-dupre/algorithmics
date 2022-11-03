//! https://protohackers.com/problem/0
//!
//! Deep inside Initrode Global's enterprise management framework lies a component that writes data
//! to a server and expects to read the same data back. (Think of it as a kind of distributed
//! system delay-line memory). We need you to write the server to echo the data back.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, info, trace};

use std::io;

use protohackers::init_logs;

const BUF_SIZE: usize = 1024 * 1024;

#[tracing::instrument(skip(socket))]
async fn client(client_id: u64, mut socket: TcpStream) -> io::Result<()> {
    info!("Connected");
    let mut buffer = Vec::with_capacity(BUF_SIZE);

    loop {
        socket.read_buf(&mut buffer).await?;

        if buffer.is_empty() {
            break;
        }

        debug!("Received {} bytes", buffer.len());
        trace!("Buffer content: {:?}", String::from_utf8_lossy(&buffer));

        while !buffer.is_empty() {
            socket.write_all(&buffer).await?;
            buffer.clear();
        }
    }

    info!("Disconnected");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
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
