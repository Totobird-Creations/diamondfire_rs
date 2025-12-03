use crate::core::{
    clone::Clone,
    macros::derive,
    marker::Copy,
    mem::ManuallyDrop
};


#[lang = "maybe_uninit"]
// #[derive(Copy)]
#[repr(transparent)]
#[rustc_pub_transparent]
pub union MaybeUninit<T> {
    uninit : (),
    value  : ManuallyDrop<T>
}

impl<T : Copy> Copy for MaybeUninit<T> { }

impl<T : Clone> Clone for MaybeUninit<T> {
    #[inline(always)]
    fn clone(&self) -> Self { Self { value : unsafe { self.value.clone() } } }
}


impl<T> MaybeUninit<T> {
    #[must_use]
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self { uninit : () }
    }
}
