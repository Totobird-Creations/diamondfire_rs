use crate::DIAMONDFIRE_Opaque;
use core::marker::PhantomData;


#[repr(transparent)]
pub struct Vec<T> {
    _opaque : *mut DIAMONDFIRE_Opaque,
    _marker : PhantomData<T>
}
