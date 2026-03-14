use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Handle one client 
async fn handle_client(mut socket: tokio::net::TcpStream) {
    // Create buffer 
    let mut buff = [0; 1024]; 

   loop {
       // Read from socket
       let n = match socket.read(&mut buff).await {
           // Handle connection  closures
          Ok(0) => return,
          Ok(n) => n,
          Err(e) => {
              eprintln!("failed to read from socket: {:?}",e);
              return ;
          }
       };

       // Write the data back
       if let Err(e) = socket.write_all(&buff[0..n]).await {
           eprintln!("failed to write to socket: {:?}", e);
           return ;
       }
   }
}

// Tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Binding the server
    let listener = TcpListener::bind("127.0.0.1:8080").await?; 

    loop {
        // Accepting connections
        let (socket, _) = listener.accept().await?;

        // Handle clients concurrently
        tokio::spawn(handle_client(socket));
    }
}
