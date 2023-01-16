extern crate tokio;
extern crate reqwest;

mod config;
mod stat;
mod output;
mod load;


use std::env;

use crate::config::Config;
use crate::output::print_stat;
use crate::load::load_test;


fn main() {
    // Prepare config
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_args(&args[1..]);

    // Run
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.threads)
        .build()
        .unwrap()
        .block_on(async {
            let stat = load_test(&config).await.unwrap();
            print_stat(&config, &stat);
        });
}
