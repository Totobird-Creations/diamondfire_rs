#![no_std]
#![no_main]

#[allow(unused_imports)]
use diamondfire::*;


#[unsafe(no_mangle)]
pub fn b(x : i32) -> i32 {
    // let y = identity(x);
    doathing(|| x + 10);
    x
}

#[unsafe(no_mangle)]
#[inline(never)]
fn doathing<F : FnOnce() -> i32>(f : F) -> i32 {
    f()
}
