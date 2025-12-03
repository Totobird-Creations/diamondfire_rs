use crate::core::intrinsics;


mod manually_drop;
pub use manually_drop::ManuallyDrop;

mod maybe_uninit;
pub use maybe_uninit::MaybeUninit;


#[inline(always)]
#[must_use]
pub const fn size_of<T>() -> usize {
    intrinsics::size_of::<T>()
}
