use crate::core::marker::{ PointeeSized, Sized, Copy, Destruct, PhantomData };


#[lang = "clone"]
pub const trait Clone : Sized {

    #[must_use]
    #[lang = "clone_fn"]
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source : &Self)
    where
        Self : [const] Destruct
    { *self = source.clone() }

}

#[rustc_builtin_macro]
pub macro Clone($item:item) {
    /* compiler built-in */
}


impl Clone for u8 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for i8 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for u16 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for i16 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for u32 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for i32 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for u64 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for i64 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for u128 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for i128 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for usize {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for isize {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for f32 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl Clone for f64 {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}

impl<T> Clone for *const T {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}
impl<T> Clone for *mut T {
    #[inline(always)]
    fn clone(&self) -> Self { *self }
}


pub struct AssertParamIsClone<T : Clone + PointeeSized> {
    _field : PhantomData<T>
}

pub struct AssertParamIsCopy<T : Copy + PointeeSized> {
    _field : PhantomData<T>
}
