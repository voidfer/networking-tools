use tokio::sync::Semaphore;
use std::sync::Arc;

use crate::config::{Config, ScanType};
use crate::result::ScanResult;
use crate::scanner::{tcp, udp, icmp};

pub async fn run_scan(config: &Config) -> Vec<ScanResult> {
    let semaphore = Arc::new(Semaphore::new(config.concurrency));

    let mut tasks = Vec::new();

    for port in config.ports.clone() {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let target = config.target.clone();
        let scan_type = config.scan_type;

        let task = tokio::spawn(async move {
            let _permit = permit;

            match scan_type {
                ScanType::Tcp => tcp::scan_port(&target, port).await,
                ScanType::Udp => udp::scan_port(&target, port).await,
                ScanType::Icmp => icmp::scan_host(&target).await,
            }
        });

        tasks.push(task);
    }

    let mut results = Vec::new();

    for task in tasks {
        if let Ok(res) = task.await {
            results.push(res);
        }
    }

    results
}
