use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

#[tokio::main]
async fn main() {
    let mut stream = UnixStream::connect("/tmp/orderbook-server.sock")
        .await
        .expect("Failed to connect to the server socket");
    stream
        .write_all(b"Hello, world!")
        .await
        .expect("Failed to write to the server socket");
}
