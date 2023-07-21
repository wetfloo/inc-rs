use std::marker::PhantomData;

use crate::prerequisite::{Storage, User};

struct UserRepository<K, V, S: Storage<K, V>> {
    storage: S,
    phantom: PhantomData<(K, V)>,
}

impl<K, V, S: Storage<K, V>> UserRepository<K, V, S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            phantom: PhantomData,
        }
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
