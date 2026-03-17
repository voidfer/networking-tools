#[derive(Debug, Clone, Copy)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

#[derive(Debug)]
pub struct ScanResult {
    pub port: u16,
    pub state: PortState,
}
