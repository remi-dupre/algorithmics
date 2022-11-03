use std::mem::size_of;

use anyhow::{Context, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn deserialize_str(mut from: impl AsyncRead + Unpin) -> Result<String> {
    let len = u8::from_be(from.read_u8().await?);
    let mut bytes = vec![0; len.into()];
    from.read_exact(&mut bytes).await?;
    bytes.iter_mut().for_each(|b| *b = u8::from_be(*b));
    String::from_utf8(bytes).context("invalid utf8 str")
}

pub async fn deserialize_roads(mut from: impl AsyncRead + Unpin) -> Result<Vec<u16>> {
    let len = u8::from_be(from.read_u8().await?);
    let mut bytes = vec![0; usize::from(len) * size_of::<u16>()];
    from.read_exact(&mut bytes).await?;

    let array = bytes
        .array_chunks()
        .map(|c| u16::from_be_bytes(*c))
        .collect();

    Ok(array)
}

pub async fn serialize_str(s: String, mut out: impl AsyncWrite + Unpin) -> Result<()> {
    let len: u8 = s.len().try_into().context(">255 bytes str")?;
    let mut bytes = s.into_bytes();
    bytes.iter_mut().for_each(|b| *b = b.to_be());
    out.write_u8(len.to_be()).await?;
    out.write_all(&bytes).await?;
    Ok(())
}
