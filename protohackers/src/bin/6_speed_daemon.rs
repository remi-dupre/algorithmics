//! https://protohackers.com/problem/6
//!
//! Motorists on Freedom Island drive as fast as they like. Sadly, this has led to a large number
//! of crashes, so the islanders have agreed to impose speed limits. The speed limits will be
//! enforced via an average speed check: Automatic Number Plate Recognition cameras will be
//! installed at various points on the road network. The islanders will use a computer system to
//! collect the data, detect cars travelling in excess of the speed limit, and send tickets to be
//! dispatched to the drivers. The islanders can't agree on which one of them should write the
//! software, so they've engaged an external contractor to do it: that's where you come in.

// Each road will be represented with two pair of channels:
//  - one for detected plates
//  - one for tickets, subscribed by each dispatcher on the road
//
// For each road, at task will be spawned which keeps track of the state of the road by consuming
// its channel and will feed tickets into dedicated channel.

use std::sync::Arc;

use protohackers::init_logs;
use protohackers::speed_daemon::state::State;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

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
    let state: Arc<State> = Arc::default();
    let listener = TcpListener::bind(format!(":::{port}")).await?;
    let mut client_count = 0;

    loop {
        let (socket, _) = listener.accept().await?;
        let state = state.clone();

        tokio::spawn(protohackers::speed_daemon::client::client(
            client_count,
            state,
            socket,
        ));

        client_count += 1;
    }
}
