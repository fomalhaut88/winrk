use std::str::FromStr;

use reqwest::Method;
use tokio::time::Duration;


#[derive(Debug, Clone)]
pub struct Config {
    pub url: String,
    pub duration: Duration,
    pub connections: usize,
    pub threads: usize,
    pub method: Method,
    pub data: String,
    pub headers: Vec<(String, String)>,
    pub timeout: Option<Duration>,
}


impl Config {
    pub fn parse_args(args: &[String]) -> Result<Self, String> {
        let mut arg_name: Option<&str> = None;

        let mut url: Option<String> = None;
        let mut duration: Duration = Duration::from_secs(10);
        let mut connections: usize = 100;
        let mut threads: usize = 1;
        let mut method = Method::GET;
        let mut data: String = "".to_owned();
        let mut headers: Vec<(String, String)> = vec![];
        let mut timeout: Option<Duration> = None;

        for arg in args {
            if (arg == "-h") || (arg == "--help") {
                print_help();
                std::process::exit(0);
            }
            if (arg == "-v") || (arg == "--version") {
                println!("{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            if arg.starts_with("-") {
                arg_name = Some(arg);
            } else {
                match arg_name {
                    None => { 
                        url = Some(arg.clone()); 
                    },
                    Some("-d") | Some("--duration") => {
                        let val: f64 = arg.parse().map_err(|_| "Duration must be float")?;
                        duration = Duration::from_secs_f64(val);
                    },
                    Some("-c") | Some("--connections") => {
                        connections = arg.parse().expect("Connections must be integer");
                    },
                    Some("-t") | Some("--threads") => {
                        threads = arg.parse().expect("Threads must be integer");
                    },
                    Some("-m") | Some("--method") => {
                        method = Method::from_str(&arg).expect("Unknown method");
                    },
                    Some("-D") | Some("--data") => {
                        data = arg.to_owned();
                    },
                    Some("-H") | Some("--header") => { 
                        let parts = arg
                            .splitn(2, ":")
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>();

                        headers.push((parts[0].clone(), parts[1].clone())); 
                    },
                    Some("-T") | Some("--timeout") => {
                        let val: f64 = arg.parse().expect("Timeout must be float");
                        timeout = Some(Duration::from_secs_f64(val));
                    },
                    _ => {},
                }
                arg_name = None;
            }
        }

        Ok(Self {
            url: url.ok_or("No URL provided")?,
            duration,
            connections,
            threads,
            method,
            data,
            headers,
            timeout,
        })
    }
}


fn print_help() {
    println!(
"{}",
"
winrk

    A command line program for load testing of an HTTP server.

Usage:

    winrk [--help | -h] 
          [--version | -v] 
          [--duration <duration> | -d <duration>] 
          [--connections <connections> | -c <connections>] 
          [--threads <threads> | -t <threads>]
          [--method <method> | -m <method>]
          [--header '<key>:<value>' | -H '<key>:<value>']
          [--data <data> | -D <data>]
          [--timeout <timeout> | -T <timeout>]
          <url>

Options:

    --help (-h)        - print this help.
    --version (-v)     - print the version.
    --duration (-d)    - duration of the test in seconds (default 10).
    --connections (-c) - number of parallel connections (default 100).
    --threads (-t)     - number of CPU workers (default 1).
    --method (-m)      - HTTP method (default GET).
    --header (-H)      - HTTP header, can be multiple (default empty).
    --data (-D)        - HTTP payload (default '').
    --timeout (-T)     - connection timeout (default empty).
    url                - URL to request.
".to_owned().trim()
    );
}
