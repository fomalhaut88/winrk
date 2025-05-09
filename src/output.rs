use tokio::time::Duration;

use crate::config::Config;
use crate::stat::Stat;


pub fn print_stat(config: &Config, stat: &Stat) {
    let mut latencies = stat.latencies.clone();
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if latencies.len() == 0 {
        println!("Error: Empty result");
        return;
    }

    let total = latencies.len();
    let err_perc = 100.0 * stat.err_count as f64 / total as f64;
    let lat_sum = latencies.iter().map(|d| d.as_secs_f64()).sum::<f64>();
    let rps = (latencies.len() * config.connections) as f64 / lat_sum;
    let lat_avg = Duration::from_secs_f64(
        lat_sum / latencies.len() as f64
    );
    let lat_med = Duration::from_secs_f64(0.5 * (
        latencies[(latencies.len() - 1) / 2].as_secs_f64() + 
        latencies[latencies.len() / 2].as_secs_f64()
    ));
    let transfers_avg = (stat.transfers * config.connections) as f64 
                        / lat_sum / 1048576f64;

    println!("Input:");
    println!("    url: {}", config.url);
    println!("    method: {}", config.method);
    println!("    threads: {}", config.threads);
    println!("    duration: {:?}", config.duration);
    println!("    connections: {}", config.connections);

    println!("");

    println!("Result:");
    println!("    total: {} requests", total);
    println!("    errors: {} errors", stat.err_count);
    println!("    error percentage: {:.1}%", err_perc);
    
    println!("    latency min: {:?}", latencies[0]);
    println!("    latency median: {:?}", lat_med);
    println!("    latency average: {:?}", lat_avg);
    println!("    latency max: {:?}", latencies[latencies.len() - 1]);
    println!("    transfers: {:.3} MB per sec", transfers_avg);
    println!("    rps: {:.1} requests per sec", rps);
}
