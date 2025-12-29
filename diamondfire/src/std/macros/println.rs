use super::format;
use diamondfire_sys::{
    df_string,
    action::DF_ACTION__Control__PrintDebug
};


pub macro println( $($tt:tt)* ) { { unsafe {
        DF_ACTION__Control__PrintDebug(
            "All" as (*const df_string),
            "No Spaces" as (*const df_string),
            "None" as (*const df_string),
            "Default" as (*const df_string),
            "Debug" as (*const df_string),
            format!( $($tt)* ).to_raw()
        );
} } }
