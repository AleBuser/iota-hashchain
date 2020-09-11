use crate::get_timestamp;
use crate::storage::models::Block;
use crate::storage::read;
use crate::types::payload::Payload;

use hex;
use sha2::{Digest, Sha256};

pub fn new_payload(data: String) -> Payload {
    let now = get_timestamp();

    let blocks: Vec<Block> = read::all_blocks().unwrap();
    let id = blocks.len() as i32 + 1;
    let previous_hash = blocks[blocks.len() - 1].hash.clone();
    let previous_iota_link = blocks[blocks.len() - 1].iota_link.clone();

    let mut hasher = Sha256::new();
    let mut concat_hash = previous_hash.clone();
    concat_hash.push_str(&data);
    concat_hash.push_str(&now.to_string());
    concat_hash.push_str(&previous_iota_link.to_string());
    hasher.update(concat_hash);

    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    Payload {
        id: id,
        timestamp: now,
        hash: hash_hex.to_string(),
        previous_hash: previous_hash.to_string(),
        previous_iota_link: previous_iota_link.to_string(),
        data: data,
    }
}
