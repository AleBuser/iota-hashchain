use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub endpoint: String,
    pub target_address: String,
    pub iota_node: String,
    pub local_pow: bool,
    pub seconds_per_block: u32,
}
