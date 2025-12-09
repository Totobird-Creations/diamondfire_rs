#![no_std]
#![no_main]


use diamondfire::prelude::*;
use diamondfire_sys::{
    df_string,
    DF_ACTION__control__PrintDebug
};


#[unsafe(no_mangle)]
pub fn collatz_conjecture(mut x : u64) {
    while (x > 0) {
        DF_ACTION__control__PrintDebug(
            df_string::from_str("All"),
            df_string::from_str("No Spaces"),
            df_string::from_str("None"),
            df_string::from_str("Default"),
            df_string::from_str("Debug"),
            df_string::from(x.to_string())
        );
        if (x % 2 == 0) {
            x /= 2;
        } else {
            x = x * 3 + 1;
        }
    }
}
