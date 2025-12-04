#![warn(missing_docs)]

#![feature(core_intrinsics)]
#![no_std]

#[doc(hidden)]
extern crate self as __private_diamondfire;
#[doc(hidden)]
pub use diamondfire_macros as __private_diamondfire_macros;


// TODO: text

// TODO: location

mod vec3;
pub use vec3::*;

// TODO: sound

// TODO: particle

// TODO: potion

// TODO: world


pub mod std;


pub mod prelude {
    pub use super::vec3::Vec3;
    pub use super::std::{ self, prelude::* };
}
