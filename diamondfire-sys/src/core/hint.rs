use crate::core::intrinsics;


pub const unsafe fn unreachable_unchecked() -> ! {
    unsafe { intrinsics::unreachable() }
}
