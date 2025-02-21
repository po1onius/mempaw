use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs::File,
    hash::Hash,
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
struct CoreKey {
    key: String,
    time_stamp: u32,
    access_times: u32,
}

#[derive(Deserialize, Serialize)]
pub struct CoreData {
    data: HashMap<CoreKey, InnerWrapValue>,
}

pub fn test() {}

impl PartialEq for CoreKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for CoreKey {}

impl Hash for CoreKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl From<&str> for CoreKey {
    fn from(value: &str) -> Self {
        CoreKey {
            key: value.to_string(),
            time_stamp: 0,
            access_times: 0,
        }
    }
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
        self.data.remove(&CoreKey::from(k));
    }

    pub fn set(&mut self, k: &str, v: &str) {
        let r = self.data.insert(
            CoreKey {
                key: k.to_string(),
                time_stamp: 0,
                access_times: 0,
            },
            InnerWrapValue::Str(v.to_string()),
        );
    }

    pub fn get(&mut self, k: &str, v: &str) -> Result<String, ()> {
        if let Some(inner) = self.data.get_mut(&CoreKey::from(k)) {
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
                return Ok(s.to_string());
            }
        }
        Err(())
    }

    pub fn hdel(&mut self, k: &str, kk: &str) -> Result<(), ()> {
        if let Some(inner) = self.data.get_mut(k) {
            if let CoreVaule {
                meta:
                    KeyMeta {
                        time_stamp: _,
                        access_times,
                    },
                value: InnerWrapValue::Hash(hm),
            } = inner
            {
                *access_times += 1;
                if hm.remove(kk).is_some() {
                    return Ok(());
                }
            }
        }
        return Err(());
    }
}
