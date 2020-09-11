pub mod handler;
pub mod security;
pub mod server;

use crate::server::security::keystore::calculate_hash;
fn is_valid(key: &str, hash: String) -> bool {
    calculate_hash(key.to_string()) == hash
}

