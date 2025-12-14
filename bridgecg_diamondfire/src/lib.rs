#![feature(
    map_try_insert,
    debug_closure_helpers
)]
#![cfg_attr(feature = "rustc", feature(rustc_private))]

#[cfg(feature = "rustc")]
extern crate rustc_middle;
#[cfg(feature = "rustc")]
extern crate rustc_span;


pub mod extern_names;

#[cfg(feature = "rustc")]
pub mod items;

#[cfg(feature = "rustc")]
pub mod mir;
