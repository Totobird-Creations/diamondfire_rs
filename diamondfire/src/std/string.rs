use crate::DIAMONDFIRE_Opaque;


#[repr(transparent)]
pub struct String {
    _opaque : *const DIAMONDFIRE_Opaque
}


pub trait ToString {
    fn to_string(&self) -> String;
}
impl ToString for u8 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for u16 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for u32 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for u64 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for u128 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for usize {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for i8 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for i16 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for i32 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for i64 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for i128 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for isize {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for f32 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for f64 {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}
impl ToString for String {
    #[inline(always)]
    fn to_string(&self) -> String { unsafe { DIAMONDFIRE_SetVariable_ToString(self) } }
}


unsafe extern "C" {
    unsafe fn DIAMONDFIRE_SetVariable_ToString(...) -> String;
}
