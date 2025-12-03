#![feature(core_intrinsics)]
#![no_std]

#[doc(hidden)]
extern crate self as __private_diamondfire;
#[doc(hidden)]
pub use diamondfire_macros as __private_diamondfire_macros;


pub mod vec3;
// pub mod player;


pub mod prelude {
    pub use super::vec3::Vec3;
    // pub use super::player::PlayerSel;
    // pub use super::std::prelude::*;
}
