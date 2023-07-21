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

pub trait Command {
    type Context: ?Sized;

    fn run(ctx: &Self::Context);
}

pub trait CommandHandler<Key, Cmd: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &Cmd, ctx: &Self::Context) -> Self::Result;
}

pub trait UserRepository<K, V> {
    fn set(&mut self, key: K, val: V);

    fn get(&self, key: &K) -> Option<&V>;

    fn remove(&mut self, key: &K) -> Option<V>;
}
