#![no_std]
#![no_main]

use diamondfire::*;


// fn sequence(mut x : u64) -> u64 {
//     // bb0
//     x += 10;
//     x
//     // return
// }


// fn if_else_stmt(mut x : u64) -> u64 {
//     // bb0
//     if (x == 0) {
//         // bb1
//         x += 2;
//     } // else { bb2 }
//     // bb3
//     if (x > 10) {
//         // bb4
//         x += 20;
//         if (x < 5) {
//             // bb5
//             x /= 2;
//         } // else { bb6 }
//         // bb7
//     } // else { bb8 }
//     // bb9
//     x -= 10;
//     x
//     // return
// }


// fn if_else_stmt(mut x : u64) -> u64 {
//     // bb0
//     if ({
//         if (x == 0) {
//             // bb1
//             x += 2;
//         } // else { bb2 }
//         // bb3
//         x > 10
//     }) {
//         // bb4
//         x += 20;
//     } else {
//         // bb5
//         if (x < 5) {
//             // bb6
//             x /= 2;
//         } else {
//             // bb7
//             x *= 2;
//         }
//         // bb8
//     }
//     // bb9
//     x -= 10;
//     x
//     // return
// }


// fn collatz_conjecture(mut x : u64) {
//     // bb0
//     while (
//         // bb1
//         x > 1
//     ) {
//         // bb2
//         if (x % 2 == 0) {
//             // bb3
//             x /= 2;
//         } else {
//             // bb4
//             x = x * 2 + 1;
//         }
//         // bb5
//     }
//     // bb6
//     // return
// }


// fn simple_loop(mut x : u64) {
//     // bb0
//     loop {
//         // bb1
//         if (x > 10) {
//             // bb2
//             x -= 10;
//         } // else { bb3 }
//         // bb4
//         x += 1;
//     }
// }


fn two_loops(mut x : u64) {
    // bb0
    loop {
        while (
            // bb1
            x < 10
        ) {
            // bb2
            x += 1;
        }
        // bb3
        x = 0;
    }
}


// fn increment_forever(mut x : u64) {
//     // bb0
//     let mut y = 10;
//     loop {
//         loop {
//             // bb1
//             x -= 1;
//         }
//         y += 1;
//     }
//     x += 10;
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
