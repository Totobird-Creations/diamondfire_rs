//! TODO: Doc comment


#![allow(unexpected_cfgs)]
#![warn(missing_docs)] // TODO: Make forbid

#![feature(
    core_intrinsics,
    decl_macro,
    formatting_options,
    str_from_raw_parts
)]

#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(doc, doc(cfg(target_os = "diamondfire")))]
#![no_std]

// #[doc(hidden)]
// extern crate self as __PRIVATE_diamondfire;
// #[doc(hidden)]
// pub use diamondfire_macros as __PRIVATE_diamondfire_macros;
// #[doc(hidden)]
// extern crate diamondfire_macros as __PRIVATE_diamondfire_macros;

// Prevents the linker from panicking due to missing `__PRIVATE_DIAMONDFIRE_SYS__EXTERN_NAMES`.
pub use diamondfire_sys as _;


pub mod value;

pub mod std;


/// Common types and functions.
pub mod prelude {
    #[doc(inline)]
    pub use diamondfire_macros::*;
    #[doc(inline)]
    pub use super::value::*;
    #[doc(inline)]
    pub use super::std::prelude::*;
}
