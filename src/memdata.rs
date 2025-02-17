use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs::File,
    io::Write,
};

use crate::config::get_config;

#[derive(Deserialize, Serialize)]
enum InerWrapValue {
    Str(String),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
    List(LinkedList<String>),
    Zset(HashMap<OrderedFloat<f32>, String>),
}

#[derive(Deserialize, Serialize)]
pub struct CoreData {
    data: HashMap<String, InerWrapValue>,
}

impl CoreData {
    pub fn dump(&self) {
        let dump_str = serde_json::to_string(self).unwrap();
        let mut dump_file = File::open(get_config().rdb_path.as_str()).unwrap();
        dump_file.write_all(dump_str.as_bytes()).unwrap();
    }
    pub fn load() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
