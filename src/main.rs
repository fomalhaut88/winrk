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
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Parse config
    match Config::parse_args(&args[1..]) {
        Ok(config) => {
            // Run
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(config.threads)
                .build()
                .unwrap()
                .block_on(async {
                    let stat_res = load_test(&config).await;

                    match stat_res {
                        Ok(stat) => { 
                            print_stat(&config, &stat); 
                        },
                        Err(err) => {
                            println!("Error: {:?}", err);
                        },
                    }            
                });
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }    
}
