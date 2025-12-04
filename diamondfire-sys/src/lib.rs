#![feature(core_intrinsics)]
#![cfg_attr(doc, feature(doc_cfg))]
#![cfg_attr(doc, doc(cfg(target_os = "diamondfire")))]
#![no_std]

#[allow(unexpected_cfgs)]
mod __target_check {
    #[cfg(not(target_os = "diamondfire"))]
    compile_error!(concat!("Unsupported compiler target `", env!("TARGET"), "` for diamondfire-sys. Please use `diamondfire-unknown-unknown`"));
}


mod ty;
pub use ty::*;

mod generated;
pub use generated::*;
