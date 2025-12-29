#![no_std]
#![no_main]

use diamondfire::prelude::*;


/// Test doc comment
#[event(PlayerJoin)]
pub fn string_test() {
    println!("{}", greet("bob"));
}

pub fn greet(name : &str) -> String {
    format!("Hello, {}!", name)
}
