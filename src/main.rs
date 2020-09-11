extern crate local;

use clokwerk::{Scheduler, TimeUnits};

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs::File};

use local::hashchain::commit_block::commit_block;
use local::hashchain::mempool::Mempool;
use local::server::security::keystore::KeyManager;
use local::server::server;
use local::types::{config::Config, transfer_config::TransferConfig};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];
    KeyManager::new(api_key.to_string());

    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();

    let mut scheduler = Scheduler::new();
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let mempool_clone = mempool.clone();
    let config_clone = config.clone();
    scheduler
        .every(config.seconds_per_block.seconds())
        .run(move || {
            commit_block(
                mempool_clone.clone(),
                TransferConfig {
                    iota_node: config_clone.iota_node.clone(),
                    target_address: config_clone.target_address.clone(),
                    local_pow: config_clone.local_pow.clone(),
                },
            );
        });
    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    server::start(config.endpoint, mempool.clone()).await
}
