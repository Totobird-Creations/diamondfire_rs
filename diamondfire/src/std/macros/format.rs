use crate::value::String;
use core::{
    fmt::{
        Display,
        Formatter,
        FormattingOptions,
        Arguments
    },
    format_args
};


#[inline(always)]
pub fn format_inner(args : Arguments) -> String {
    let mut s = String::new();
    let mut f = Formatter::new(
        &mut s,
        FormattingOptions::new()
    );
    Display::fmt(&args, &mut f).unwrap();
    s
}


pub macro format( $($tt:tt)* ) { {
    format_inner(format_args!( $($tt)* ))
} }
