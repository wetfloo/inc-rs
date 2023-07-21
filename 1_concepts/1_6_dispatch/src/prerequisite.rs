use std::borrow::Cow;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}
