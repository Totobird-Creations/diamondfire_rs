use core::marker::PhantomData;

use diamondfire_sys::{
    df_opaque,
    df_list,
    action::{
        DF_ACTION__SetVar__CreateList,
        DF_ACTION__SetVar__AppendValue
    }
};

pub struct Vec<T> {
    _opaque  : *mut df_list,
    _phantom : PhantomData<T>,
}

impl<T> Vec<T> {

    /// SAFETY: You must guarantee the inner values of the list are T.
    pub unsafe fn from_raw(inner : *mut df_list) -> Vec<T> {
        Vec {
            _opaque  : inner,
            _phantom : PhantomData
        }
    }

    pub fn new() -> Vec<T> { unsafe {
        let mut out = MaybeUninit::<df_list>::uninit();
        DF_ACTION__SetVar__CreateList(out.as_mut_ptr());
        Self::from_raw(out.assume_init())
    } }

    pub fn push(&mut self, value: T) { unsafe {
        DF_ACTION__SetVar__AppendValue(
            self.inner,
            (&raw const value) as (*const df_opaque)
        )
    } }

    pub fn get(&self, index: usize) -> Option<&T> {
        todo!("cannot be implemented until pointer representation is decided")
    }

}
