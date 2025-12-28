#![feature(
    map_try_insert,
    debug_closure_helpers
)]
#![cfg_attr(feature = "rustc_private", feature(rustc_private))]

#[cfg(feature = "rustc_private")]
extern crate rustc_middle;
#[cfg(feature = "rustc_private")]
extern crate rustc_span;


#[cfg(feature = "extern_names")]
pub mod extern_names;

#[cfg(feature = "bridge_items")]
pub mod bridge_items;

#[cfg(feature = "dfmir")]
pub mod dfmir;

#[cfg(feature = "dflir")]
pub mod dflir;

mod common;
pub use common::*;
