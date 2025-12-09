//! TODO: Doc comment


#![allow(unexpected_cfgs)]
#![warn(missing_docs)] // TODO: Make forbid

#![feature(
    core_intrinsics,
    decl_macro,
    formatting_options
)]

#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(doc, doc(cfg(target_os = "diamondfire")))]
#![no_std]

#[doc(hidden)]
extern crate self as __private_diamondfire;
#[doc(hidden)]
pub use diamondfire_macros as __private_diamondfire_macros;


pub mod value;
pub mod debug;

pub mod std;


/// Common types and functions.
pub mod prelude {
    #[doc(inline)]
    pub use super::value::*;
    #[doc(inline)]
    pub use super::debug::println;
    #[doc(inline)]
    pub use super::std::prelude::*;
}
