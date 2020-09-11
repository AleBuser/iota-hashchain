use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub data: &'static str,
    pub timestamp: i32,
}
