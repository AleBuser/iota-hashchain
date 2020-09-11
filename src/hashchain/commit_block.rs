use crate::hashchain::mempool::Mempool;
use crate::hashchain::new_payload::new_payload;
use crate::storage::models::{Block, NewBlock};
use crate::storage::{read, write};
use crate::types::{payload::Payload, transfer_config::TransferConfig};
use base64::{encode_config, URL_SAFE_NO_PAD};
use std::sync::{Arc, Mutex};

use iota_client::options::SendTransferOptions;
use iota_conversion::trytes_converter;
use iota_lib_rs::prelude::*;
use iota_model::Transfer;

pub fn commit_block(mempool: Arc<Mutex<Mempool>>, config: TransferConfig) {
    let payload = new_payload(
        mempool
            .lock()
            .expect("couldn't lock mempool")
            .get_entries_stringified(),
    );
    mempool.lock().expect("couldn't lock mempool").flush();

    let iota_link = send_payload(&payload, config);
    println!("Commited Payload nr. {} at tx: {}", payload.id, iota_link);

    write::write_block(NewBlock {
        timestamp: payload.timestamp,
        hash: payload.hash,
        iota_link: iota_link,
        previous_hash: payload.previous_hash,
        previous_iota_link: payload.previous_iota_link,
        data: payload.data,
    });

    let blocks: Vec<Block> = read::all_blocks().unwrap();
    let last_block = &blocks[blocks.len() - 1];
    println!(
        "Stored Block nr. {} with hash: {}",
        last_block.id, last_block.hash
    );
}

fn send_payload(payload: &Payload, config: TransferConfig) -> String {
    let payload_string = serde_json::to_string(payload).unwrap();
    let message =
        trytes_converter::to_trytes(&encode_config(&payload_string, URL_SAFE_NO_PAD)).unwrap();

    let transfer = Transfer {
        address: config.target_address.clone(),
        message,
        ..Transfer::default()
    };
    let mut api = iota_client::Client::new(&config.iota_node);
    let tx = api
        .send_transfers(
            transfer,
            &config.target_address,
            SendTransferOptions {
                local_pow: config.local_pow,
                threads: 2,
                ..SendTransferOptions::default()
            },
        )
        .unwrap();

    tx[0].hash.clone()
}
