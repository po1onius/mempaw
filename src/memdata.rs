use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::OrderedFloat;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet, LinkedList},
    fs::{File, OpenOptions},
    hash::Hash,
    io::{Read, Write},
};

use crate::config::get_config;

#[derive(BorshDeserialize, BorshSerialize)]
enum InnerWrapValue {
    Str(String),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
    List(LinkedList<String>),
    Zset(HashMap<OrderedFloat<f32>, String>),
}

#[derive(BorshDeserialize, BorshSerialize, Default, PartialOrd, Ord, Eq)]
struct CoreKey {
    key: String,
    time_stamp: Cell<u32>,
    access_times: Cell<u32>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CoreData {
    data: HashMap<CoreKey, InnerWrapValue>,
}

pub fn test() {}

impl PartialEq for CoreKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Hash for CoreKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl From<&str> for CoreKey {
    fn from(value: &str) -> Self {
        CoreKey {
            key: value.to_string(),
            time_stamp: Cell::new(0),
            access_times: Cell::new(0),
        }
    }
}

impl CoreData {
    pub fn dump(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&get_config().rdb_path)
            .unwrap();
        let bin = borsh::to_vec(&self).unwrap();
        let _ = file.write_all(&bin).unwrap();
    }

    pub fn load() -> Self {
        let mut file = File::open(&get_config().rdb_path).unwrap();
        let mut bin_buf = Vec::new();
        let _ = file.read_to_end(&mut bin_buf).unwrap();
        borsh::from_slice(&bin_buf).unwrap()
    }

    pub fn del(&mut self, k: &str) {
        self.data.remove(&CoreKey::from(k));
    }

    pub fn set(&mut self, k: &str, v: &str) {
        let r = self.data.insert(
            CoreKey {
                key: k.to_string(),
                time_stamp: Cell::new(0),
                access_times: Cell::new(0),
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
