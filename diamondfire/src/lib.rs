#![warn(missing_docs)]

#![feature(
    core_intrinsics,
    decl_macro,
    formatting_options
)]
#![no_std]

#[doc(hidden)]
extern crate self as __private_diamondfire;
#[doc(hidden)]
pub use diamondfire_macros as __private_diamondfire_macros;


pub mod debug;

// TODO: text

mod location;
pub use location::*;

mod vec3;
pub use vec3::*;

// TODO: sound

// TODO: particle

// TODO: potion

// TODO: world

// TODO: extras::Vec2
// TODO: extras::Vec4
// TODO: extras::Dir2
// TODO: extras::Dir3
// TODO: extras::Dir4
// TODO: extras::IVec2
// TODO: extras::IVec3
// TODO: extras::IVec4
// TODO: extras::BVec2
// TODO: extras::BVec3
// TODO: extras::BVec4
// TODO: extras::Mat2
// TODO: extras::Mat3
// TODO: extras::Mat4
// TODO: extras::Quat
// TODO: extras::LargeVec
// TODO: extras::LargeHashMap
// TODO: extras::Combined // Player or entity
// TODO: extras::CombinedSel


pub mod std;


/// Common types and functions.
pub mod prelude {
    pub use super::debug::println;
    pub use super::vec3::Vec3;
    pub use super::location::Location;
    pub use super::std::prelude::*;
}


// #[inline(always)]
// pub fn inlined(x : u64) -> u64 { // TODO: Remove
//     if (x > 10) {
//         x / 2
//     } else {
//         x * 2
//     }
// }
