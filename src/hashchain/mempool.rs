use crate::get_timestamp;
use crate::types::entry::Entry;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Mempool {
    entries: Vec<Entry>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add(&mut self, new_data: String) -> () {
        let new_entry = Entry {
            data: string_to_static_str(new_data),
            timestamp: get_timestamp(),
        };
        self.entries.push(new_entry);
    }

    pub fn flush(&mut self) -> () {
        self.entries = vec![];
    }

    pub fn get_entries(&mut self) -> Vec<Entry> {
        self.entries.clone()
    }

    pub fn get_entries_stringified(&mut self) -> String {
        serde_json::to_string(&self.entries.clone()).unwrap()
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
