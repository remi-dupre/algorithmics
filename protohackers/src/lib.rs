#![feature(array_chunks)]
pub mod speed_daemon;

use anyhow::Result;
use futures::Stream;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_logs() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env()
        .expect("could not build filter");

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting global subscriber failed");
}

pub fn split_at_bytes<'a, R: AsyncRead + 'a>(
    bytes: &'a [u8],
    reader: R,
) -> impl Stream<Item = Result<Vec<u8>>> + 'a {
    let reader = Box::pin(reader);

    futures::stream::try_unfold(
        (Vec::new(), reader),
        move |(mut buffer, mut reader)| async move {
            let end_pos = {
                let mut prev_len = 0;

                loop {
                    let end_pos = buffer
                        .iter()
                        .enumerate()
                        .skip(prev_len)
                        .find(|(_, b)| bytes.contains(b))
                        .map(|(i, _)| i);

                    if let Some(pos) = end_pos {
                        break pos;
                    }

                    prev_len = buffer.len();

                    if reader.read_buf(&mut buffer).await? == 0 {
                        return Ok(None);
                    }
                }
            };

            let res = buffer.drain(..=end_pos).collect();
            Ok(Some((res, (buffer, reader))))
        },
    )
}
