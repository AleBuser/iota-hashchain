extern crate local;

use local::get_timestamp;
use local::storage::models::NewBlock;
use local::storage::write;

fn main() {
    let genesis_block = NewBlock {
        timestamp: get_timestamp(),
        hash: String::default(),
        iota_link: String::default(),
        previous_hash: String::default(),
        previous_iota_link: String::default(),
        data: "Genesis Block".to_string(),
    };

    write::write_block(genesis_block);
}
