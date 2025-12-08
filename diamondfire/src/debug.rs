use diamondfire_sys::{
    df_string,
    DF_ACTION__control__PrintDebug
};


pub macro println( $($tt:tt)* ) { {
    let args = format_args!( $($tt)* );
    if let Some(known) = args.as_str() { unsafe {
        DF_ACTION__control__PrintDebug(
            df_string::from_str("All"),
            df_string::from_str("No Spaces"),
            df_string::from_str("None"),
            df_string::from_str("Default"),
            df_string::from_str("Debug"),
            known
        );
    } }
} }
