#![no_std]
#![no_main]

use diamondfire::prelude::*;


/// Test doc comment
#[event(PlayerJoin)]
pub fn fibonacci() {
    let mut a = 1usize;
    let mut b = 1usize;
    for _i in 0..10 {
        let c = a + b;
        unsafe { diamondfire_sys::action::DF_ACTION__Control__PrintDebug( // TODO: Replace with println
            "All".as_ptr() as *const diamondfire_sys::df_string,
            "No Spaces".as_ptr() as *const diamondfire_sys::df_string,
            "None".as_ptr() as *const diamondfire_sys::df_string,
            "Default".as_ptr() as *const diamondfire_sys::df_string,
            "Debug".as_ptr() as *const diamondfire_sys::df_string,
            c
        ); }
        a = b;
        b = c;
    };
}
