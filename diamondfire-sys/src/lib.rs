#![feature(
    no_core,
    lang_items,
    intrinsics,
    rustc_attrs,
    auto_traits,
    transparent_unions,
    const_trait_impl,
    const_destruct,
    decl_macro,
    generic_const_exprs,
    specialization
)]
#![no_core]
#![no_std]


mod core;
pub use core::*;

pub mod diamondfire {

    mod ty;
    pub use ty::*;

    mod generated;
    pub use generated::*;

}
