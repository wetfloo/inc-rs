use crate::prerequisite::{Storage, UserRepository};

struct UserRepositoryImpl<K, V> {
    storage: Box<dyn Storage<K, V>>,
}

impl<K, V> UserRepositoryImpl<K, V> {
    pub fn new(storage: Box<dyn Storage<K, V>>) -> Self {
        Self { storage }
    }
}

impl<K, V> UserRepository<K, V> for UserRepositoryImpl<K, V> {
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
