use std::marker::PhantomData;

use crate::prerequisite::{Storage, User};

struct UserRepository<K, V> {
    storage: Box<dyn Storage<K, V>>,
}

impl<K, V> UserRepository<K, V> {
    pub fn new(storage: Box<dyn Storage<K, V>>) -> Self {
        Self { storage }
    }

    pub fn set(&mut self, key: K, val: V) {
        self.storage.set(key, val);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key)
    }
}
