use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;

    socket.write_all(b"Hello server!").await?;

    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("Server replied: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
