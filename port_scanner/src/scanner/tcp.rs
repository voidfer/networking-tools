use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::result::{ScanResult, PortState};

pub async fn scan_port(target: &str, port: u16) -> ScanResult {
    let addr = format!("{}:{}", target, port);

    let result = timeout(Duration::from_millis(500), TcpStream::connect(&addr)).await;

    let state = match result {
        Ok(Ok(_)) => PortState::Open,
        Ok(Err(_)) => PortState::Closed,
        Err(_) => PortState::Filtered,
    };

    ScanResult { port, state }
}
