use diamondfire_sys::{
    df_string,
    DF_ACTION__control__PrintDebug
};


pub macro println( $($tt:tt)* ) { {
    unsafe {
        DF_ACTION__control__PrintDebug(
            df_string::from_str("All"),
            df_string::from_str("No Spaces"),
            df_string::from_str("None"),
            df_string::from_str("Default"),
            df_string::from_str("Debug"),
            df_string::from(format!( $($tt)* ))
        );
    }
} }
