use crate::storage::models::Block;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Payload {
    pub id: i32,
    pub timestamp: i32,
    pub hash: String,
    pub previous_hash: String,
    pub previous_iota_link: String,
    pub data: String,
}

pub fn from_block(block: &Block) -> Payload {
    Payload {
        id: block.id,
        timestamp: block.timestamp,
        hash: block.hash.clone(),
        previous_hash: block.previous_hash.clone(),
        previous_iota_link: block.previous_iota_link.clone(),
        data: block.data.clone(),
    }
}
