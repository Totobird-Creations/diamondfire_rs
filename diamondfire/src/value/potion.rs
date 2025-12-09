use diamondfire_sys::df_potion;


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Potion {
    _opaque : df_potion
}

impl Potion {
    #[inline(always)]
    pub fn raw(self) -> df_potion { self._opaque }
    #[inline(always)]
    pub fn from_raw(raw : df_potion) -> Self { Self { _opaque : raw } }
}
