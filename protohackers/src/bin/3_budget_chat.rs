use std::pin::Pin;

use anyhow::Result;
use futures::{FutureExt, Stream, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

use protohackers::{init_logs, split_at_bytes};

#[derive(Clone, Debug)]
enum OpKind {
    Join,
    Leave,
    Message(String),
}

#[derive(Clone)]
struct Op {
    client: u64,
    kind: OpKind,
}

struct Client {
    id: u64,
    name: Option<String>,
    tcp_in: Pin<Box<dyn Stream<Item = Result<Vec<u8>>>>>,
    tcp_out: OwnedWriteHalf,
}

impl Client {
    async fn new(id: u64, tcp: TcpStream) -> Result<Self> {
        let (tcp_in, mut tcp_out) = tcp.into_split();

        let tcp_in = Box::pin(split_at_bytes(&[b'\n'], tcp_in).map(|msg| {
            let mut msg = msg?;
            msg.pop();
            Ok(msg)
        }));

        tcp_out
            .write_all(b"Welcome to chat, please enter your name\n")
            .await?;

        Ok(Self {
            id,
            name: None,
            tcp_in,
            tcp_out,
        })
    }

    async fn listen(&mut self) -> Result<OpKind> {
        let msg = match self.tcp_in.next().await {
            Some(x) => x?,
            None => {
                return Ok(OpKind::Leave);
            }
        };

        let msg = String::from_utf8_lossy(&msg).to_string();

        Ok({
            if self.name.is_none() {
                if msg.is_empty() || msg.chars().any(|c| !c.is_alphanumeric()) {
                    OpKind::Leave
                } else {
                    self.name = Some(msg);
                    OpKind::Join
                }
            } else {
                OpKind::Message(msg)
            }
        })
    }

    async fn message(&mut self, from: &Client, msg: &str) -> Result<()> {
        let from = from.name.as_deref().unwrap_or("anonymous");
        let data = format!("[{from}] {msg}\n");
        self.tcp_out.write_all(data.as_bytes()).await?;
        Ok(())
    }

    async fn joined(&mut self, new: &Client) -> Result<()> {
        let name = new.name.as_deref().unwrap_or("anonymous");
        let data = format!("* {name} joined chat\n");
        self.tcp_out.write_all(data.as_bytes()).await?;
        Ok(())
    }

    async fn left(&mut self, new: &Client) -> Result<()> {
        let name = new.name.as_deref().unwrap_or("anonymous");
        let data = format!("* {name} left chat\n");
        self.tcp_out.write_all(data.as_bytes()).await?;
        Ok(())
    }

    async fn welcome(&mut self, others: impl IntoIterator<Item = &Client>) -> Result<()> {
        let others_str: String = others
            .into_iter()
            .filter_map(|c| c.name.as_deref())
            .collect::<Vec<_>>()
            .join(", ");

        let data = format!("* welcome to the room, connected: {others_str}\n");
        self.tcp_out.write_all(data.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Default)]
struct Server {
    clients: Vec<Client>,
}

impl Server {
    async fn listen(&mut self) -> Result<Op> {
        if self.clients.is_empty() {
            return futures::future::pending().await;
        }

        let (res, _, _) = futures::future::select_all(self.clients.iter_mut().map(|client| {
            Box::pin(async move {
                client.listen().await.map(|kind| Op {
                    client: client.id,
                    kind,
                })
            })
        }))
        .await;

        res
    }

    async fn message(&mut self, from: u64, msg: &str) -> Result<()> {
        let (from, others): (Vec<_>, Vec<_>) = self
            .clients
            .iter_mut()
            .filter(|client| client.name.is_some())
            .partition(|client| client.id == from);

        let from = from.into_iter().next().expect("Sender is not registered");
        info!(id = from.id, name = from.name, msg = msg, "Message sent");
        let futs = others.into_iter().map(|client| client.message(from, msg));
        futures::future::try_join_all(futs).await?;
        Ok(())
    }

    async fn joined(&mut self, new_id: u64) -> Result<()> {
        let (new, mut old): (Vec<_>, Vec<_>) = self
            .clients
            .iter_mut()
            .partition(|client| client.id == new_id);

        let new = new
            .into_iter()
            .next()
            .expect("New client is not registered");

        info!(id = new.id, name = new.name, "Joined");

        futures::future::try_join_all(
            old.iter_mut()
                .filter(|client| client.name.is_some())
                .map(|client| client.joined(new)),
        )
        .await?;

        new.welcome(old.iter().map(|x| &**x)).await?;
        Ok(())
    }

    async fn left(&mut self, left_id: u64) -> Result<()> {
        let (left, mut others): (Vec<_>, Vec<_>) = self
            .clients
            .iter_mut()
            .partition(|client| client.id == left_id);

        let left = left
            .into_iter()
            .next()
            .expect("New client is not registered");

        info!(id = left.id, name = left.name, "Left");

        if left.name.is_some() {
            futures::future::try_join_all(
                others
                    .iter_mut()
                    .filter(|client| client.name.is_some())
                    .map(|client| client.left(left)),
            )
            .await?;
        }

        let pos = self
            .clients
            .iter()
            .enumerate()
            .find(|(_, client)| client.id == left_id)
            .unwrap()
            .0;

        self.clients.swap_remove(pos);
        Ok(())
    }
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
    let mut clients_count = 0;
    let listener = TcpListener::bind(format!(":::{port}")).await?;
    let mut server = Server::default();

    loop {
        let mut fut_new_clients = Box::pin(listener.accept().fuse());
        let mut fut_fetch_op = Box::pin(server.listen().fuse());

        futures::select! {
            tcp_client = fut_new_clients  => {
                std::mem::drop(fut_fetch_op);
                let (tcp, _) = tcp_client?;

                info!(id = clients_count, "Connected");
                server.clients.push(Client::new(clients_count, tcp).await?);
                clients_count+=1;
            }
            op = fut_fetch_op  => {
                std::mem::drop(fut_fetch_op);
                let op = op?;

                match op.kind {
                    OpKind::Join => server.joined(op.client).await?,
                    OpKind::Leave => server.left(op.client).await?,
                    OpKind::Message(msg) => server.message(op.client, &msg).await?,
                }
            }
        }
    }
}
