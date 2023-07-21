use rand::{thread_rng, Rng};
use std::{marker::PhantomData, vec};

pub trait FactTeller {
    fn fact(&self) -> &'static str;
}

trait FactListHolder {
    fn list_facts() -> Vec<&'static str>;
}

pub struct Fact<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Fact<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

/// Blanket implementation: anything that can give a list of facts
/// can also tell one
impl<Holder: FactListHolder> FactTeller for Holder {
    fn fact(&self) -> &'static str {
        let facts = Self::list_facts();
        let idx = thread_rng().gen_range(0..facts.len());
        facts[idx]
    }
}

impl<I> FactListHolder for Fact<Vec<I>> {
    fn list_facts() -> Vec<&'static str> {
        vec!["Vec is heap-allocated.", "Vec may re-allocate on growing."]
    }
}

impl FactListHolder for Fact<i32> {
    fn list_facts() -> Vec<&'static str> {
        vec![
            "i32 is Copy, so it will be cloned implicitly.",
            "i32 is Sized, and is allocated on the stack.",
        ]
    }
}
