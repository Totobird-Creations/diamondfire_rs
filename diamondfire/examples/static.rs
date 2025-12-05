#![no_std]
#![no_main]

use diamondfire::*;


static mut GLOBAL_COUNTER : u64 = 0;


fn increment_value(v : u64) -> u64 {
    return v + 1;
}

// fn increment_counter() {
//     unsafe { GLOBAL_COUNTER += 1; }
// }

fn decrement_value(v : u64) -> u64 {
    return v - 1;
}

// fn decrement_counter() {
//     unsafe { GLOBAL_COUNTER -= 1; }
// }

// fn panic_test() {
//     panic!("hello");
// }

fn assert_test(v : u64) {
    assert!(v > 10);
}
