use std::collections::HashMap;

use super::Value;

pub struct Primary {
    store: HashMap<String, Value>,
}

impl Primary {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: Value) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.store.get(key)
    }

    pub fn del(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn list(&self) -> Vec<String> {
        self.store.keys().map(|k| k.to_string()).collect()
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }
}