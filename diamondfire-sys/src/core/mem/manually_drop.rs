use crate::core::{
    clone::Clone,
    macros::derive,
    marker::{ Copy, Sized },
    ptr
};


#[lang = "manually_drop"]
#[derive(Clone, Copy)]
#[repr(transparent)]
// #[rustc_pub_transparent]
pub struct ManuallyDrop<T : ?Sized> {
    value : T
}

impl<T> ManuallyDrop<T> {
    #[inline(always)]
    pub const fn new(value : T) -> ManuallyDrop<T> {
        ManuallyDrop { value }
    }
}

impl<T : ?Sized> ManuallyDrop<T> {
    #[inline]
    pub unsafe fn drop(slot : &mut ManuallyDrop<T>) {
        unsafe { ptr::drop_in_place(&mut slot.value) }
    }
}
