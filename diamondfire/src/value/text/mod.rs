use diamondfire_sys::{
    df_string,
    df_text,
    DF_ACTION__setSPECIALSpace_variable__StyledText,
    DF_ACTION__setSPECIALSpace_variable__ParseMiniMessage
};
use core::mem::MaybeUninit;

mod style;
pub use style::*;


#[derive(Clone)]
#[repr(transparent)]
pub struct Text {
    _opaque : df_text
}

impl Text {
    #[inline(always)]
    pub fn raw(self) -> df_text { self._opaque }
    #[inline(always)]
    pub fn from_raw(raw : df_text) -> Self { Self { _opaque : raw } }
}


impl Text {

    #[inline(always)]
    pub fn literal(s : &str) -> Self { unsafe {
        let mut out = MaybeUninit::<df_text>::uninit();
        DF_ACTION__setSPECIALSpace_variable__StyledText(
            df_string::from_str("No spaces"),
            df_string::from_str("False"),
            df_string::from_str(s)
        );
        Self::from_raw(out.assume_init())
    } }

    // TODO: keybind

    // TODO: lang

    // TODO: lang_or

}
