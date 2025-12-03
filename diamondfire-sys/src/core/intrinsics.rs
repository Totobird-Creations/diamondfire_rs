#[rustc_intrinsic]
pub const unsafe fn transmute_unchecked<Src, Dst>(src : Src) -> Dst;

#[rustc_intrinsic]
pub const unsafe fn unreachable() -> !;

#[rustc_intrinsic]
pub const fn size_of<T>() -> usize;
