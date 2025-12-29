//! Partial reimplementation of `std` for DiamondFire.


use core::{
    intrinsics::abort,
    panic::PanicInfo
};


// #[cfg(feature = "allocator")]
// pub mod alloc;

// #[cfg(feature = "allocator")]
// pub mod boxed;

// pub mod string;

pub mod macros;


/// Common types and functions.
pub mod prelude {

    // #[cfg(feature = "allocator")]
    // #[doc(inline)]
    // pub use super::boxed::Box;

    // #[doc(inline)]
    // pub use super::string::{ String, ToString };

    pub use super::macros::{
        format,
        println
    };

}


#[panic_handler]
fn handle_panics(_info : &PanicInfo) -> ! {
    abort();
}
