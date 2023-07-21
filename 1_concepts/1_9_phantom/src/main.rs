mod fact;

use fact::{Fact, FactTeller};

fn main() {
    let f: Fact<Vec<String>> = fact::Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());

    let f: Fact<i32> = fact::Fact::new();
    println!("Fact about i32: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
}
