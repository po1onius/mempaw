use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, LinkedList},
    default,
    fs::File,
    io::Write,
};

use crate::config::get_config;

#[derive(Deserialize, Serialize)]
enum InnerWrapValue {
    Str(String),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
    List(LinkedList<String>),
    Zset(HashMap<OrderedFloat<f32>, String>),
}

#[derive(Deserialize, Serialize, Default)]
struct KeyMeta {
    time_stamp: u32,
    access_times: u32,
}

#[derive(Deserialize, Serialize)]
struct CoreVaule {
    meta: KeyMeta,
    value: InnerWrapValue,
}

#[derive(Deserialize, Serialize)]
pub struct CoreData {
    data: HashMap<String, CoreVaule>,
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

    pub fn del(&mut self, k: &str) {
        self.data.remove(k);
    }

    pub fn set(&mut self, k: &str, v: &str) {
        let r = self.data.insert(
            k.to_string(),
            CoreVaule {
                value: InnerWrapValue::Str(v.to_string()),
                meta: KeyMeta::default(),
            },
        );
    }

    pub fn get(&mut self, k: &str, v: &str) -> String {
        if let Some(inner) = self.data.get_mut(k) {
            if let CoreVaule {
                meta:
                    KeyMeta {
                        time_stamp: _,
                        access_times,
                    },
                value: InnerWrapValue::Str(s),
            } = inner
            {
                *access_times += 1;
                return s.to_string();
            }
        }
        "".into()
    }
}
