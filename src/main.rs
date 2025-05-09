mod config;
mod stat;
mod load;
mod output;

use clap::Parser;

use crate::config::Config;
use crate::load::load_test;
use crate::output::print_stat;


fn main() {
    let config = Config::parse();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.threads)
        .build()
        .unwrap()
        .block_on(async {
            match load_test(&config).await {
                Ok(stat) => { 
                    print_stat(&config, &stat); 
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                },
            }            
        });
}
