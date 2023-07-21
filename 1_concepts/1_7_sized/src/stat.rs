use std::marker::PhantomData;

use crate::prerequisite::{Storage, UserRepository};

struct UserRepositoryImpl<K, V, S> {
    storage: S,
    phantom: PhantomData<(K, V)>,
}

impl<K, V, S: Storage<K, V>> UserRepositoryImpl<K, V, S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            phantom: PhantomData,
        }
    }
}

impl<K, V, S: Storage<K, V>> UserRepository<K, V> for UserRepositoryImpl<K, V, S> {
    fn set(&mut self, key: K, val: V) {
        self.storage.set(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key)
    }
}
