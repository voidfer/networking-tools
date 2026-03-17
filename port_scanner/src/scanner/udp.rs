use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};

use crate::result::{ScanResult, PortState};

pub async fn scan_port(target: &str, port: u16) -> ScanResult {
    let addr = format!("{}:{}", target, port);

    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

    let _ = socket.send_to(&[0], &addr).await;

    let mut buf = [0u8; 1024];

    let result = timeout(
        Duration::from_millis(1000),
        socket.recv_from(&mut buf),
    )
    .await;

    let state = match result {
        Ok(Ok(_)) => PortState::Open,
        Ok(Err(_)) => PortState::Closed,
        Err(_) => PortState::Filtered,
    };

    ScanResult { port, state }
}
