use tokio::time::Duration;


#[derive(Debug, Clone)]
pub struct Stat {
    pub latencies: Vec<Duration>,
    pub err_count: usize,
    pub transfers: usize,
}


impl Stat {
    pub fn new() -> Self {
        Self {
            latencies: Vec::new(),
            err_count: 0,
            transfers: 0,
        }
    }
}
