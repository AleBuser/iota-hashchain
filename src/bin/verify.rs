extern crate local;

use base64::{decode_config, URL_SAFE_NO_PAD};
use chrono::prelude::*;
use hex;
use iota_conversion::trytes_converter;
use iota_lib_rs::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::File;

use local::storage::read;
use local::types::config::Config;
use local::types::payload::{from_block, Payload};

fn main() -> () {
    let mut blocks = read::all_blocks().unwrap();

    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let node_url = &config.iota_node.clone();
    let mut api = iota_client::Client::new(node_url);

    let mut payloads = Vec::new();
    payloads.push(Some(from_block(&blocks[0])));

    let mut on_tangle = 0;

    blocks.reverse();
    for block in &blocks {
        let iota_link = block.iota_link.clone();
        if iota_link != String::default() {
            let payload = {
                let tx = api
                    .get_trytes(&vec![iota_link])
                    .unwrap()
                    .trytes()
                    .clone()
                    .unwrap()[0]
                    .clone();
                match tx[..9] != *"999999999" {
                    true => match decode_iota_tx(tx) {
                        Some(payload) => {
                            on_tangle = on_tangle + 1;
                            Some(payload)
                        }
                        None => None,
                    },
                    false => Some(from_block(&block)),
                }
            };
            payloads.push(payload);
        }
    }

    if payloads.len() <= 2 {
        println!("Hashchain is empty...");
        return;
    };

    println!("Genesis block nr. 1 is valid!");

    payloads.reverse();
    let items = payloads.len() - 2usize;

    for idx in 0..items {
        let current_payload = payloads[idx + 1].clone();
        if current_payload.clone().is_none() {
            println!(
                "Undecoded transaction: {}, aborting verification...",
                idx + 1
            );
            break;
        }
        let current_payload = current_payload.unwrap();

        let hash_is_correct = verify_current_hash(&current_payload);

        let prev_payload = payloads[idx].clone();
        if prev_payload.clone().is_none() {
            println!("Undecoded transaction: {}, aborting verification...", idx);
            break;
        }
        let prev_payload = prev_payload.unwrap();

        let hashes_match = prev_payload.hash == current_payload.previous_hash;

        if !hash_is_correct {
            println!("Invalid hash on block: {}", current_payload.id)
        };
        if !hashes_match {
            println!(
                "Unmatched hash link on blocks: {} to {}",
                &current_payload.id, &prev_payload.id
            )
        };
        if hash_is_correct && hashes_match {
            println!("Block nr. {} is valid!", prev_payload.id)
        };
    }
    println!(
        "Found {}/{} valid blocks on Tangle",
        on_tangle,
        &blocks.len() - 1
    );
    if on_tangle != 0 {
        println!(
            "Deepest block found in Tangle was stored at time (UTC): {} ",
            timestamp_to_string(blocks[on_tangle - 1].timestamp)
        )
    };
}

fn verify_current_hash(payload: &Payload) -> bool {
    let presumed_hash = payload.hash.clone();

    let mut hasher = Sha256::new();
    let mut concat_hash = payload.previous_hash.clone();
    concat_hash.push_str(&payload.data);
    concat_hash.push_str(&payload.timestamp.to_string());
    concat_hash.push_str(&payload.previous_iota_link.to_string());
    hasher.update(concat_hash);

    let hash = hasher.finalize();
    let result_hash = hex::encode(hash);

    presumed_hash == result_hash
}

fn decode_iota_tx(payload: String) -> Option<Payload> {
    let message = &payload[..2188];
    message.to_string().push_str("9");

    let raw = trytes_converter::to_string(&message.to_string()).unwrap();
    let decoded_tx = &decode_config(&raw, URL_SAFE_NO_PAD);
    match decoded_tx {
        Ok(tx) => Some(serde_json::from_slice(tx).unwrap()),
        Err(_) => {
            println!("Error decoding Transaction, possibly trying to decode a bundle");
            None
        }
    }
}

fn timestamp_to_string(timestamp: i32) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
