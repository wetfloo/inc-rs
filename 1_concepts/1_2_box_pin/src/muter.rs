use crate::sayhi::SayHi;
use std::{fmt::Debug, pin::Pin};

pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
    // Implementation must be meaningful, and
    // obviously call something requiring `&mut self`.
    // The point here is to practice dealing with
    // `Pin<&mut Self>` -> `&mut self` conversion
    // in different contexts, without introducing
    // any `Unpin` trait bounds.
}

impl<T: Debug> SayHi for T {}

impl<T: Ord> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Don't move data after getting a mutable reference to it, otherwise
        // things will go south
        let this = unsafe { self.get_unchecked_mut() };
        // If something holds a reference to data in the `Vec` by index, it's
        // gonna get a different piece of data after sorting
        //
        // This might be is unexpected, but shouldn't be unsafe
        this.sort_unstable();
    }
}
