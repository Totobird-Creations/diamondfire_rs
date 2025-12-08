use super::alloc::{ alloc, dealloc };
use core::{
    mem::MaybeUninit,
    ptr::drop_in_place
};


#[repr(transparent)]
pub struct Box<T : ?Sized> {
    ptr : *mut T
}


impl<T> Box<T> {

    #[inline(always)]
    pub fn new(x : T) -> Box<T> { unsafe { // TODO: Special case for ZST
        let ptr = alloc() as *mut T;
        *ptr = x;
        Box { ptr }
    } }

    #[inline(always)]
    pub fn new_uninit() -> Box<MaybeUninit<T>> {
        Box::new(MaybeUninit::uninit())
    }

}


impl<T : ?Sized> Drop for Box<T> {
    #[inline(always)]
    fn drop(&mut self) { unsafe {
        drop_in_place(self.ptr);
        dealloc(self.ptr as *mut u8)
    } }
}
