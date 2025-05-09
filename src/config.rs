use std::num::ParseFloatError;

use clap::Parser;
use reqwest::Method;
use tokio::time::Duration;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// URL to load
    pub url: String,

    /// Load duration
    #[arg(short, long, default_value = "10", value_parser = parse_duration)]
    pub duration: Duration,

    /// Number of simultaneous connections
    #[arg(short, long, default_value_t = 100)]
    pub connections: usize,

    /// Number of CPU threads
    #[arg(short, long, default_value_t = 1)]
    pub threads: usize,

    /// HTTP method
    #[arg(short, long, default_value = "GET")]
    pub method: Method,

    /// Request body
    #[arg(short = 'D', long, default_value = "")]
    pub data: String,

    /// HTTP header
    #[arg(short = 'H', long, value_parser = parse_headers)]
    pub header: Vec<(String, String)>,

    /// Request timeout
    #[arg(short = 'T', long, value_parser = parse_duration)]
    pub timeout: Option<Duration>,
}


fn parse_duration(arg: &str) -> Result<Duration, ParseFloatError> {
    let secs: f64 = arg.parse()?;
    Ok(Duration::from_secs_f64(secs))
}


fn parse_headers(arg: &str) -> Result<(String, String), String> {
    let parts = arg.splitn(2, ":")
                   .map(|s| s.trim().to_string())
                   .collect::<Vec<String>>();
    if parts.len() == 2 {
        Ok((parts[0].clone(), parts[1].clone()))
    } else {
        Err("invalid header".to_string())
    }
}
