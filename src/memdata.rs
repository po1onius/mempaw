use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::OrderedFloat;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet, LinkedList},
    fs::{File, OpenOptions},
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

#[derive(BorshDeserialize, BorshSerialize, Default)]
struct CoreKeyMeta {
    time_stamp: Cell<u32>,
    access_times: Cell<u32>,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CoreValue {
    meta: CoreKeyMeta,
    value: InnerWrapValue,
}

#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct CoreData {
    data: HashMap<String, CoreValue>,
}

pub fn test() {}

impl CoreValue {
    fn access(&self) {
        self.meta.access_times.set(self.meta.access_times.get() + 1);
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
        self.data.remove(k).unwrap();
    }

    pub fn set(&mut self, k: &str, v: &str) {
        if self.data.get(k).is_some() {
            self.data.remove(k).unwrap();
        }
        self.data.insert(
            k.to_string(),
            CoreValue {
                meta: CoreKeyMeta::default(),
                value: InnerWrapValue::Str(v.to_string()),
            },
        );
    }

    pub fn get(&self, k: &str) -> Result<&str, ()> {
        if let Some(CoreValue {
            meta:
                CoreKeyMeta {
                    time_stamp: _,
                    access_times,
                },
            value: InnerWrapValue::Str(s),
        }) = self.data.get(k)
        {
            access_times.set(access_times.get() + 1);
            Ok(s)
        } else {
            Err(())
        }
    }

    pub fn hdel(&mut self, k: &str, kk: &str) -> Result<(), ()> {
        if let Some(CoreValue {
            meta: _,
            value: InnerWrapValue::Hash(m),
        }) = self.data.get_mut(k)
        {
            if m.remove(kk).is_some() {
                return Ok(());
            }
        }
        Err(())
    }

    pub fn hset(&mut self, k: &str, kk: &str, v: &str) {
        match self.data.get_mut(k) {
            Some(inner) => match &mut inner.value {
                InnerWrapValue::Hash(m) => {
                    if let Some(cur_val) = m.get_mut(kk) {
                        *cur_val = v.to_string();
                    } else {
                        m.insert(kk.to_string(), v.to_string());
                    }
                }
                _ => {
                    let m = HashMap::from([(kk.to_string(), v.to_string())]);
                    *inner = CoreValue {
                        meta: CoreKeyMeta::default(),
                        value: InnerWrapValue::Hash(m),
                    };
                }
            },
            None => {
                let m = HashMap::from([(kk.to_string(), v.to_string())]);
                let new_val = CoreValue {
                    meta: CoreKeyMeta::default(),
                    value: InnerWrapValue::Hash(m),
                };
                self.data.insert(k.to_string(), new_val);
            }
        }
    }
}
