use futures::TryStreamExt;
use serde_json::Value;
use tokio::io::AsyncRead;
use tokio::net::UnixListener;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::SymmetricallyFramed;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

#[tokio::main]
async fn main() {
    let listener =
        UnixListener::bind("/tmp/orderbook-server.sock").expect("Failed to bind the unix socket");
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Accepted new connection");
                tokio::spawn(async move {
                    read_frame(stream).await;
                });
            }
            Err(e) => eprintln!("connection failed: {}", e),
        }
    }
}

async fn read_frame<T: AsyncRead + Unpin + Send + 'static>(io: T) {
    let transport = FramedRead::new(io, LengthDelimitedCodec::new());
    let mut frames = SymmetricallyFramed::new(transport, SymmetricalJson::<Value>::default());

    // Spawn a task that prints all received messages to STDOUT
    tokio::spawn(async move {
        while let Some(msg) = frames.try_next().await.unwrap() {
            println!("GOT: {:?}", msg);
        }
    });
}
