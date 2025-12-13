use crate::value::String;
use diamondfire_sys::{
    df_string,
    df_sound,
    DF_SOUND__Any,
    DF_ACTION__setSPECIALSpace_variable__SetSoundType,
    DF_ACTION__setSPECIALSpace_variable__GetSoundType,
    DF_ACTION__setSPECIALSpace_variable__SetSoundVariant,
    DF_ACTION__setSPECIALSpace_variable__GetSoundVariant,
    DF_ACTION__setSPECIALSpace_variable__SetCustomSound,
    DF_ACTION__setSPECIALSpace_variable__GetCustomSound
};
use core::mem::MaybeUninit;


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sound {
    _opaque : df_sound
}

impl Sound {
    #[inline(always)]
    pub fn raw(self) -> df_sound { self._opaque }
    #[inline(always)]
    pub fn from_raw(raw : df_sound) -> Self { Self { _opaque : raw } }
}


impl Sound {

    #[inline(always)]
    pub fn by_name(name : &str) -> Self { unsafe {
        let mut out = MaybeUninit::<df_sound>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetSoundType(
            out.as_mut_ptr(),
            DF_SOUND__Any(),
            df_string::from_str(name)
        );
        Self::from_raw(out.assume_init())
    } }

    #[inline(always)]
    pub fn by_name_variant(name : &str, variant : &str) -> Self { unsafe {
        let mut out = MaybeUninit::<df_sound>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetSoundType(
            out.as_mut_ptr(),
            DF_SOUND__Any(),
            df_string::from_str(name)
        );
        DF_ACTION__setSPECIALSpace_variable__SetSoundVariant(
            out.as_mut_ptr(),
            out.assume_init(),
            df_string::from_str(variant)
        );
        Self::from_raw(out.assume_init())
    } }

    #[inline(always)]
    pub fn by_key(key : &str) -> Self { unsafe {
        let mut out = MaybeUninit::<df_sound>::uninit();
        DF_ACTION__setSPECIALSpace_variable__SetCustomSound(
            out.as_mut_ptr(),
            DF_SOUND__Any(),
            df_string::from_str(key)
        );
        Self::from_raw(out.assume_init())
    } }

}


impl Sound {

    // TODO: pub fn name(self) -> Option<String>;

    // TODO: pub fn variant(self) -> Option<String>;

    // TODO: pub fn name_variant(self) -> Option<(String, String,)>;

    // TODO: pub fn key(self) -> Option<String>;

    pub unsafe fn name_unchecked(self) -> String { unsafe {
        let mut out = MaybeUninit::<df_string>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetSoundType(
            out.as_mut_ptr(),
            self._opaque
        );
        String::from_raw(out.assume_init())
    } }

    pub unsafe fn variant_unchecked(self) -> String { unsafe {
        let mut out = MaybeUninit::<df_string>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetSoundVariant(
            out.as_mut_ptr(),
            self._opaque
        );
        String::from_raw(out.assume_init())
    } }

    pub unsafe fn key_unchecked(self) -> String { unsafe {
        let mut out = MaybeUninit::<df_string>::uninit();
        DF_ACTION__setSPECIALSpace_variable__GetCustomSound(
            out.as_mut_ptr(),
            self._opaque
        );
        String::from_raw(out.assume_init())
    } }

}
