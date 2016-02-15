//! Please do not "use zoom::*" or the zoom::Box type will override std::boxed::Box!
//!
//! This crate creates traits for driving particle physics functions and doing simple
//! vector geometry. It also contains various particle interactions in the particle module, which
//! is publically used in zoom.

pub mod vector;
pub mod particle;
pub use vector::*;
pub use particle::*;
