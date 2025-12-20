#![no_std]
#![no_main]

use diamondfire::prelude::*;


struct A {
    x : i64,
    y : u32,
    b : B
}
impl Drop for A {
    fn drop(&mut self) {
        dropping_a();
    }
}

struct B {
    z : u16
}
impl Drop for B {
    fn drop(&mut self) {
        dropping_b();
    }
}


#[unsafe(no_mangle)]
pub fn a() {
    let a = A { x : 0, y : 1, b : B { z : 2 } };
    print_a(&a);
}


unsafe extern "C" {
    safe fn print_a(a : &A);
    safe fn dropping_a();
    safe fn dropping_b();
}
