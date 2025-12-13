#![no_std]
#![no_main]

#[allow(unused_parens)]
use diamondfire::*;


#[unsafe(no_mangle)]
pub fn add_1k(a : i32) -> i64 {
    add_large(a, 1000)
}

unsafe extern "C" {
    pub safe fn add_large(x : i32, y : i32) -> i64;
}
