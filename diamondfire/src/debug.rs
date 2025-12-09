pub macro println( $($tt:tt)* ) { {
    unsafe {
        diamondfire_sys::DF_ACTION__control__PrintDebug(
            diamondfire_sys::df_string::from_str("All"),
            diamondfire_sys::df_string::from_str("No Spaces"),
            diamondfire_sys::df_string::from_str("None"),
            diamondfire_sys::df_string::from_str("Default"),
            diamondfire_sys::df_string::from_str("Debug"),
            diamondfire_sys::df_string::from(format!( $($tt)* ))
        );
    }
} }
