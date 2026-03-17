use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum ScanType {
    Tcp,
    Udp,
    Icmp,
}

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub ports: RangeInclusive<u16>,
    pub concurrency: usize,
    pub scan_type: ScanType,
}

pub fn parse_ports(input: &str) -> RangeInclusive<u16> {
    if let Some((start, end)) = input.split_once('-') {
        let s: u16 = start.parse().unwrap();
        let e: u16 = end.parse().unwrap();
        s..=e
    } else {
        let p: u16 = input.parse().unwrap();
        p..=p
    }
}

fn parse_scan_type(s: &str) -> ScanType {
    match s.to_lowercase().as_str() {
        "udp" => ScanType::Udp,
        "icmp" => ScanType::Icmp,
        _ => ScanType::Tcp,
    }
}

impl Config {
    pub fn new(
        target: String,
        ports: String,
        concurrency: usize,
        scan_type: String,
    ) -> Self {
        let range = parse_ports(&ports);

        Self {
            target,
            ports: range,
            concurrency,
            scan_type: parse_scan_type(&scan_type),
        }
    }
}
