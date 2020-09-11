use crate::storage::schema::block;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Block {
    pub id: i32,
    pub timestamp: i32,
    pub hash: String,
    pub iota_link: String,
    pub previous_hash: String,
    pub previous_iota_link: String,
    pub data: String,
}

#[derive(Insertable, Clone)]
#[table_name = "block"]
pub struct NewBlock {
    pub timestamp: i32,
    pub hash: String,
    pub iota_link: String,
    pub previous_hash: String,
    pub previous_iota_link: String,
    pub data: String,
}
