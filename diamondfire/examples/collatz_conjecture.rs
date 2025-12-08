#![no_std]
#![no_main]


use diamondfire::prelude::*;


#[unsafe(no_mangle)]
pub fn collatz_conjecture(mut x : u64) {
    while (x > 0) {
        println!("{}", x);
        if (x % 2 == 0) {
            x /= 2;
        } else {
            x = x * 3 + 1;
        }
    }
}
