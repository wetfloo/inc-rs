use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, RwLock, TryLockError},
};

type Internal<T> = Arc<RwLock<Vec<T>>>;
pub type Result<T> = std::result::Result<T, ThreadInteractionError>;

// Since TryLockError seems to contain some data that I don't really want to
// deal with for the purposes of this code example, I'm just gonna remap it to
// the enum with no data
#[derive(Debug)]
pub enum ThreadInteractionError {
    Poisoned,
    WouldBlock,
}

impl Display for ThreadInteractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poisoned => "other thread panicked, this thing is poisoned",
            Self::WouldBlock => "couldn't acquire a lock",
        }
        .fmt(f)
    }
}

impl<T> From<TryLockError<T>> for ThreadInteractionError {
    fn from(value: TryLockError<T>) -> Self {
        match value {
            TryLockError::Poisoned(_) => Self::Poisoned,
            TryLockError::WouldBlock => Self::WouldBlock,
        }
    }
}

impl Error for ThreadInteractionError {}

#[derive(Default)]
// Implicitly Send and Sync
pub struct GlobalStack<T> {
    internal: Internal<T>,
}

/// Thread-safe implementation of **Stack** data structure, using `Vec` under
/// the hood
///
/// # Cloning
/// Cloning this does stack doesn't create much of an overhead, since it just
/// clones the internal `Arc`
impl<'a, T> GlobalStack<T> {
    /// Pops the topmost value, returning it in a `Result<Option<T>>`.
    /// `Option` is `None`, if there weren't any values available in a stack.
    pub fn pop(&mut self) -> Result<Option<T>> {
        self.write(|vec| vec.pop())
    }

    /// Pushes the new value to the stack, adding it and allowing it to later be
    /// popped
    pub fn push(&mut self, value: T) -> Result<()> {
        self.write(move |vec| vec.push(value))
    }

    /// Returns the current length of the stack
    pub fn len(&self) -> Result<usize> {
        self.read(|vec| vec.len())
    }

    fn new_ref(&self) -> Internal<T> {
        self.internal.clone()
    }

    fn write<F, R>(&self, writer: F) -> Result<R>
    where
        F: FnOnce(&mut Vec<T>) -> R,
    {
        let r = self.new_ref();
        let mut res = r.try_write().map_err(ThreadInteractionError::from)?;

        Ok(writer(&mut res))
    }

    fn read<F, R>(&self, reader: F) -> Result<R>
    where
        F: FnOnce(&Vec<T>) -> R,
    {
        let r = self.new_ref();
        let res = r.try_read().map_err(ThreadInteractionError::from)?;

        Ok(reader(&res))
    }
}

impl<T: Copy> GlobalStack<T> {
    pub fn yank(&self) -> Result<Option<T>> {
        self.read(|vec| vec.last().copied())
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        let internal = self.new_ref();
        Self { internal }
    }
}

mod tests {
    use std::{thread, time::Duration};

    use super::GlobalStack;

    fn create_values<T: Default>(len: usize) -> Vec<T> {
        (0..len).map(|_| T::default()).collect()
    }

    #[test]
    fn add_remove_values() {
        let mut stack = GlobalStack::default();
        let len = 10;
        let values: Vec<i32> = create_values(len);
        for value in values {
            stack
                .push(value)
                .expect(&format!("Failed to push value {}", value));
        }

        assert_eq!(len, stack.len().expect("Couldn't get stack's len"))
    }

    #[test]
    fn lock_threads() {
        let stack = GlobalStack::default();
        let mut handles = Vec::new();

        let mut clone = stack.clone();
        let handle = thread::spawn(move || clone.push("Thread 1"));
        handles.push(handle);

        let mut clone = stack.clone();
        let handle = thread::spawn(move || {
            // Assuming that on any modern day computer spawning a thread and
            // completing a push operation wouldn't take longer than this
            //
            // Maybe testing multithreaded code like this isn't a good idea...
            thread::sleep(Duration::from_millis(10));
            clone.push("Thread 2")
        });
        handles.push(handle);

        for handle in handles {
            handle.join().unwrap().unwrap();
        }
        assert_eq!(2, stack.len().unwrap());
    }

    #[test]
    fn yank() {
        let mut stack = GlobalStack::default();

        stack.push(69).unwrap();
        stack.push(420).unwrap();

        assert_eq!(420, stack.yank().unwrap().unwrap());
    }
}
