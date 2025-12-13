#![no_std]
#![no_main]

use diamondfire::prelude::*;


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


// fn really_simple_loop(mut x : u64) {
//     loop {
//         x += 10;
//     }
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


// fn two_loops(mut x : u64) {
//     // bb0
//     loop {
//         while (
//             // bb1
//             x < 10
//         ) {
//             // bb2
//             x += 1;
//         }
//         // bb3
//         x = 0;
//     }
// }


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


// fn simple_match(mut x : u64) {
//     // bb0
//     match (x) {
//         1 => {
//             // bb4
//             x += 10;
//         },
//         2 => {
//             // bb3
//             x -= 2;
//         },
//         3 => {
//             // bb2
//             x *= 5;
//         },
//         4 => {
//             // bb5
//             x /= 3;
//         },
//         _ => {
//             // bb1
//             x = 999;
//         }
//     }
//     // bb6
//     // return
// }


// fn while_breaks(mut x : u64) {
//     // bb0
//     let mut y = 100;
//     while (
//         // bb1
//         y > 50
//     ) {
//         // bb2
//         if (x % 2 == 0) {
//             // bb3
//             y /= 2;
//             if (x == 0) {
//                 // bb4
//                 break; // => bb11
//             } else {
//                 // bb5
//                 y += 1;
//             }
//         } else {
//             // bb6
//             y = y * 3 + 1;
//             if (x < 2) {
//                 // bb7
//                 break; // => bb11
//             } else {
//                 // bb8
//                 y -= 1;
//             }
//         }
//         // bb9
//         x -= 1;
//     } // exit { bb10 }
//     // bb11
//     // bb12
//     y += 1;
//     // return
// }


// fn loop_breaks(mut x : u64) {
//     // bb0
//     let mut y = 100;
//     loop {
//         // bb1
//         if (x % 2 == 0) {
//             // bb2
//             y /= 2;
//             if (x == 0) {
//                 // bb3
//                 break;
//             } else {
//                 // bb4
//                 y += 1;
//             }
//         } else {
//             // bb5
//             y = y * 3 + 1;
//             if (x < 2) {
//                 // bb6
//                 break;
//             } else {
//                 // bb7
//                 y -= 1;
//             }
//         }
//         // bb8
//         x -= 1;
//     }
//     // bb9
//     y += 1;
//     // return
// }


// fn for_loops(mut x : u64) {
//     // bb0
//     let iter = 1..x;
//     // bb1
//     for i in iter { // for ( bb2; bb3 )
//         // bb5+7
//         x /= i;
//     } // unreachable { bb4 }
//     // bb6
// }


// fn nested_for_loops(mut size : u64) -> u64 {
//     let mut out = 0;
//     let y_iter = 0..size;
//     for y in y_iter {
//         let x_iter = 0..y;
//         for x in x_iter {
//             out += x*y;
//         }
//     }
//     out
// }
