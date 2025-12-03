use crate::core::clone::Clone;


mod copy;
pub use copy::Copy;


#[lang = "pointee_sized"]
pub trait PointeeSized { }

#[lang = "meta_sized"]
pub trait MetaSized : PointeeSized { }

#[lang = "sized"]
pub trait Sized : MetaSized { }


#[lang = "freeze"]
pub unsafe auto trait Freeze { }


#[lang = "destruct"]
pub const trait Destruct { }


#[lang = "phantom_data"]
pub struct PhantomData<T : PointeeSized>;
impl<T : PointeeSized> Clone for PhantomData<T> {
    #[inline(always)]
    fn clone(&self) -> Self { Self }
}
impl<T : PointeeSized> Copy for PhantomData<T> { }
