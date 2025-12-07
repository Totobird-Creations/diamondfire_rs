#![no_std]
#![no_main]

use diamondfire::*;


static mut GLOBAL_COUNTER : u64 = 0;


// fn sequence(mut x : u64) -> u64 {
//     x += 10;
//     x
// }


fn if_else_stmt(mut x : u64) -> u64 {
    // bb0
    if (x == 0) {
        // bb1
        x += 2;
    }
    // bb2
    // bb3
    if (x > 10) {
        // bb4
        x += 20;
        if (x < 5) {
            // bb5
            x /= 2;
        }
        // bb6
        // bb7
    } // else { bb8 }
    // bb9
    x -= 10;
    x
    // return
}


// fn if_else_stmt(mut x : u64) -> u64 {
//     if ({
//         if (x == 0) {
//             x += 2;
//         }
//         x > 10
//     }) {
//         x += 20;
//     } else {
//         if (x < 5) {
//             x /= 2;
//         } else {
//             x *= 2;
//         }
//     }
//     x -= 10;
//     x
// }


// fn increment_forever(mut x : u64) {
//     let mut y = 10;
//     'yl : loop {
//         'xl: loop {
//             x -= 1;
//             if (y > 10) {
//                 break 'yl;
//             }
//             if (x > 10) {
//                 continue;
//             }
//             break;
//         }
//         y += 1;
//     }
//     x += 10;
// }
