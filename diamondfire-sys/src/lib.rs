#![allow(unexpected_cfgs)]

#![feature(core_intrinsics)]
#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(doc, doc(cfg(target_os = "diamondfire")))]
#![no_std]


#[cfg(all(not(diamondfiresys_docsrs), not(doc), not(rust_analyzer), not(target_os = "diamondfire")))]
compile_error!(concat!("`diamondfire-sys` does not support the `", env!("TARGET"), "` target. Please use `diamondfire-unknown-unknown`"));


#[forbid(missing_docs)]
mod ty;
pub use ty::*;

#[forbid(missing_docs)]
mod generated;
pub use generated::*;
