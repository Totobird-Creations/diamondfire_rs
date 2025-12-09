#![no_std]
#![no_main]


use diamondfire::prelude::*;
use diamondfire_sys::{
    df_string,
    DF_ACTION__control__PrintDebug
};


#[unsafe(no_mangle)]
pub fn collatz_conjecture(mut x : u64) {
    // bb0
    while (
        // bb1
        x > 0
    ) {
        // bb15
        unsafe { DF_ACTION__control__PrintDebug(
            // bb2
            df_string::from_str("All"),
            // bb9
            df_string::from_str("No Spaces"),
            // bb10
            df_string::from_str("None"),
            // bb11
            df_string::from_str("Default"),
            // bb12
            df_string::from_str("Debug"),
            // bb14
            df_string::from_str(
                // bb13, bb3
                &x.to_string()
            )
        ); }
        // bb4
        if (x % 2 == 0) {
            // bb5
            x /= 2;
        } else {
            // bb6
            x = x * 3 + 1;
        }
        // bb7
    }
    // bb8
    // return
}
