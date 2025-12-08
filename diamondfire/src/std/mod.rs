//! Partial reimplementation of `std` for DiamondFire.


use core::{
    intrinsics::abort,
    panic::PanicInfo
};


#[cfg(feature = "allocator")]
pub mod alloc;
#[cfg(feature = "allocator")]
pub mod boxed;


/// Common types and functions.
pub mod prelude {
    #[cfg(feature = "allocator")]
    pub use super::boxed;
}


#[panic_handler]
fn handle_panics(_info : &PanicInfo) -> ! {
    abort();
}
