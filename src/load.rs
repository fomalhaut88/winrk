use std::sync::Arc;
use std::time::Instant;

use reqwest::{Client, Response};
use tokio::{task, time::sleep, sync::Mutex};

use crate::config::Config;
use crate::stat::Stat;


async fn perform_request(client: &Client, config: &Config) -> Result<Response, reqwest::Error> {
    // Create request builder
    let mut builder = client
        .request(config.method.clone(), &config.url)
        .body(config.data.clone());

    // Add headers
    for (key, value) in config.headers.iter() {
        builder = builder.header(key, value);
    }

    // Add timeout
    if let Some(duration) = config.timeout {
        builder = builder.timeout(duration);
    }

    // Perform request
    let resp = builder.send().await?;

    // Return status
    Ok(resp)
}


pub async fn load_test(config: &Config) -> Result<Stat, reqwest::Error> {
    // Thread handlers
    let mut handlers = Vec::new();

    // Statistics
    let stat_arc = Arc::new(Mutex::new(Stat::new()));

    // Create parallel tasks
    for _ in 0..config.connections {
        // Clone statistics
        let config_clone = config.clone();
        let stat_clone = Arc::clone(&stat_arc);

        // Spawn a new thread
        let handler = task::spawn(async move {
            // Create a client
            let client = Client::new();

            loop {
                // Perform request
                let start = Instant::now();
                let resp_result = perform_request(&client, &config_clone).await;
                let latency = start.elapsed();

                // Get stat
                let mut stat = stat_clone.lock().await;

                // Add latency
                stat.latencies.push(latency);

                // Add transfers and error count
                match resp_result {
                    Ok(resp) => {
                        if resp.status().is_client_error() || 
                           resp.status().is_server_error() {
                            stat.err_count += 1;
                        }

                        stat.transfers += resp.text().await
                            .unwrap_or("".to_owned()).len();
                    },
                    Err(_) => {
                        stat.err_count += 1;
                    },
                }
            }
        });
        handlers.push(handler);
    }

    // Wait
    sleep(config.duration).await;

    // Abort threads
    for handler in handlers {
        handler.abort();
    }

    // Return stat
    let stat = stat_arc.lock().await;
    Ok(stat.clone())
}
