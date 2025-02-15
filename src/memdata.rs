use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, LinkedList};

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
    pub fn dump(&self) {}
    pub fn load() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
