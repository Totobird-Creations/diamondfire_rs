use super::String;
use core::fmt::{
    Formatter,
    FormattingOptions,
    Display
};


pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for T
where
    T : Display + ?Sized
{
    #[inline]
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut f = Formatter::new(&mut s, FormattingOptions::new());
        Display::fmt(self, &mut f)
            .expect("a Display implementation returned an error unexpectedly");
        s
    }
}
