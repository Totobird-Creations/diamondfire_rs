#![no_std]
#![no_main]

use diamondfire::prelude::*;


struct A {
    x : i64,
    y : u32
}
impl Drop for A {
    fn drop(&mut self) {
        dropping_a();
    }
}


#[unsafe(no_mangle)]
pub fn a() {
    let a = A { x : 0, y : 1 };
    print_a(a);
}


unsafe extern "C" {
    safe fn print_a(a : A);
    safe fn dropping_a();
}
