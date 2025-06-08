//! Common library for shanten number calculation algorithm implementations.
//!
//! This crate provides common traits, types, macros, and utilities for implementing and testing
//! various shanten number calculation algorithms in Mahjong.
//!
//! # Usage
//!
//! - Implement your own algorithm by implementing the [`ShantenCalculator`] trait.
//! - Use the [`shanten_tests!`] macro to automatically generate standard test cases
//!   for your implementation.
//! - See the `dummy` crate for a minimal example.
//!
//! Most users only need to implement [`ShantenCalculator`] and
//! use the [`shanten_tests!`] macro for testing.
//! Internal utilities such as [`TileCountsExt`] are used by macros and
//! do not need to be called directly.

#![warn(missing_docs)]

mod constants;
mod test_macros;
mod test_utils;
mod traits;
mod types;

pub use constants::*;
pub use test_utils::*;
pub use traits::*;
pub use types::*;
