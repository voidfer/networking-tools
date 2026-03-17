use tokio::process::Command;

use crate::result::{ScanResult, PortState};

pub async fn scan_host(target: &str) -> ScanResult {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg("1")
        .arg(target)
        .output()
        .await;

    let state = match output {
        Ok(o) if o.status.success() => PortState::Open,
        Ok(_) => PortState::Filtered,
        Err(_) => PortState::Closed,
    };

    // ICMP is host-level → fake port 0
    ScanResult { port: 0, state }
}
