use std::sync::Arc;
use std::time::Instant;

use reqwest::{Client, RequestBuilder};
use tokio::{task, time::sleep, sync::Mutex};

use crate::config::Config;
use crate::stat::Stat;


fn create_request_builder(config: &Config) -> RequestBuilder {
    // Create a client
    let client = Client::new();

    // Create request builder
    let mut builder = client
        .request(config.method.clone(), &config.url)
        .body(config.data.clone());

    // Add headers
    for (key, value) in config.header.iter() {
        builder = builder.header(key, value);
    }

    // Add timeout
    if let Some(duration) = config.timeout {
        builder = builder.timeout(duration);
    }

    builder
}


pub async fn load_test(config: &Config) -> Result<Stat, reqwest::Error> {
    // Thread handlers
    let mut handlers = Vec::new();

    // Statistics
    let stat_arc = Arc::new(Mutex::new(Stat::new()));

    // Create parallel tasks
    for _ in 0..config.connections {
        // Clone statistics
        let stat_clone = Arc::clone(&stat_arc);

        // Create a request builder
        let request_builder = create_request_builder(&config);

        // Spawn a new thread
        let handler = task::spawn(async move {
            loop {
                // Create request builder copy
                let request_builder_copy = request_builder.try_clone().unwrap();

                // Start timer
                let start = Instant::now();

                // Send request
                let resp_result = request_builder_copy.send().await;

                // Download the body
                let (err, body_size) = if let Ok(resp) = resp_result {
                    let err = resp.status().is_client_error() || 
                              resp.status().is_server_error();
                    let body_size = resp.text().await.map(|body| body.len())
                                                     .unwrap_or(0);
                    (err, body_size)
                } else {
                    (true, 0)
                };

                // Finish timer
                let latency = start.elapsed();

                // Get stat
                let mut stat = stat_clone.lock().await;

                // Modify stat
                stat.latencies.push(latency);
                stat.err_count += err as usize;
                stat.transfers += body_size;
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
