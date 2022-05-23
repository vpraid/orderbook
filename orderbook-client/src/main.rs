use anyhow::Result;
use futures::SinkExt;
use serde_json::json;
use tokio::io::AsyncWrite;
use tokio::net::UnixStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::SymmetricallyFramed;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

#[tokio::main]
async fn main() {
    let stream = UnixStream::connect("/tmp/orderbook-server.sock")
        .await
        .expect("Failed to connect to the server socket");
    write_frame(stream)
        .await
        .expect("Failed to write to the server socket");
}

async fn write_frame<T: AsyncWrite + Unpin>(io: T) -> Result<()> {
    let transport = FramedWrite::new(io, LengthDelimitedCodec::new());
    let mut framed = SymmetricallyFramed::new(transport, SymmetricalJson::default());
    framed
        .send(json!({ "type": "subscribe", "channel": "orderbook" }))
        .await?;
    Ok(())
}
