//! TODO: Doc comment


#![allow(unexpected_cfgs)]
#![warn(missing_docs)] // TODO: Forbid

#![feature(
    extern_types
)]

#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(doc, doc(cfg(target_os = "diamondfire")))]
#![no_std]


#[cfg(all(doc, not(diamondfire_doc)))]
compile_error!(concat!("`cargo doc` for diamondfire-sys must be run with `RUSTFLAGS=\"--cfg diamondfire_doc\" RUSTDOCFLAGS=\"--cfg diamondfire_doc\"`"));

#[cfg(all(not(diamondfire_doc), not(doc), not(rust_analyzer), not(target_os = "diamondfire")))]
compile_error!(concat!("diamondfire-sys does not support the `", env!("TARGET"), "` target. use `diamondfire-unknown-unknown`"));

#[cfg(all(diamondfire_doc, target_os = "diamondfire"))]
compile_error!("diamondfire-sys docs should not be compiled for `diamondfire-unknown-unknown`");


mod ty;
pub use ty::*;

pub mod var;

pub mod consts;

mod generated;
pub use generated::*;
