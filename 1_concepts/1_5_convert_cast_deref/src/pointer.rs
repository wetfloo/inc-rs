use std::ops::Deref;

use rand::random;

type Internal<T> = [T; VALUES_COUNT];
#[derive(PartialEq, Eq)]
struct Random<T>(Internal<T>);

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let idx = random::<usize>() % VALUES_COUNT;
        &self.0[idx]
    }
}

impl<T> Random<T> {
    pub fn from_values(v1: T, v2: T, v3: T) -> Self {
        Random([v1, v2, v3])
    }
}

impl<T> From<Internal<T>> for Random<T> {
    fn from(value: Internal<T>) -> Self {
        Random(value)
    }
}

const VALUES_COUNT: usize = 3;

#[cfg(test)]
mod tests {
    use super::Random;

    fn random() -> Random<i32> {
        VALUES.into()
    }

    #[test]
    fn store_get() {
        assert!(VALUES.contains(&*random()));
    }

    const VALUES: [i32; 3] = [1, 2, 3];
}
