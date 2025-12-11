#![no_std]
#![no_main]

use diamondfire::*;


#[inline(never)]
pub fn identity<A>(v : A) -> A { // TODO: Remove
    return v;
}

#[unsafe(no_mangle)]
pub fn a(x : f32) -> f32 {
    identity(x)
}

// #[unsafe(no_mangle)]
pub fn b(x : i32) -> i32 {
    identity(x)
}
