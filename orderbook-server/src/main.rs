use orderbook_common::{Command, SOCKET};

use futures::TryStreamExt;
use tokio::io::AsyncRead;
use tokio::net::UnixListener;
use tokio::sync::mpsc;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::SymmetricallyFramed;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

mod server;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(async move {
        server::run(rx).await;
    });
    let listener = UnixListener::bind(SOCKET).expect("Failed to bind the unix socket");
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Accepted new connection");
                let tx = tx.clone();
                tokio::spawn(async move {
                    read_frames(stream, tx).await;
                });
            }
            Err(e) => eprintln!("connection failed: {}", e),
        }
    }
}

async fn read_frames<T: AsyncRead + Unpin + Send + 'static>(io: T, tx: mpsc::Sender<Command>) {
    let transport = FramedRead::new(io, LengthDelimitedCodec::new());
    let mut frames = SymmetricallyFramed::new(transport, SymmetricalJson::<Command>::default());
    while let Some(msg) = frames.try_next().await.unwrap() {
        tx.send(msg).await.unwrap();
    }
}
