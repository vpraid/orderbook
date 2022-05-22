use tokio::io::AsyncReadExt;
use tokio::net::{UnixListener, UnixStream};

#[tokio::main]
async fn main() {
    let listener =
        UnixListener::bind("/tmp/orderbook-server.sock").expect("Failed to bind the unix socket");
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("Accepted new connection");
                tokio::spawn(async move {
                    process(stream).await;
                });
            }
            Err(e) => eprintln!("connection failed: {}", e),
        }
    }
}

async fn process(mut stream: UnixStream) {
    let mut buf = [0; 1024];
    let n = stream
        .read(&mut buf)
        .await
        .expect("Failed to read from stream");
    println!("{}", String::from_utf8_lossy(&buf[..n]));
}
