#![no_std]
#![no_main]

use diamondfire::prelude::*;


/// Test doc comment
#[event(PlayerJoin)]
pub fn string_test() {
    unsafe { diamondfire_sys::action::DF_ACTION__Control__PrintDebug( // TODO: Replace with println
        "All".as_ptr() as *const diamondfire_sys::df_string,
        "No Spaces".as_ptr() as *const diamondfire_sys::df_string,
        "None".as_ptr() as *const diamondfire_sys::df_string,
        "Default".as_ptr() as *const diamondfire_sys::df_string,
        "Debug".as_ptr() as *const diamondfire_sys::df_string,

        "Hello, World!".as_ptr() as *const diamondfire_sys::df_string
    ); }
}
