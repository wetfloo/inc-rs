use std::fmt;
use std::pin::Pin;

pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
