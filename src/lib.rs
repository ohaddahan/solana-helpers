//! # Solana Helpers
//!
//! A collection of utility functions and helpers for Solana program development.
//! This crate provides common functionality for account management, transfers,
//! conversions, and validation operations.

// #![warn(missing_docs, unreachable_pub)]
// #![deny(unused_must_use, rust_2018_idioms)]
// #![doc(test(no_crate_inject, attr(deny(warnings, rust_2018_idioms)), allow(dead_code, unused_variables)))]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

/// Error types used throughout the library.
pub mod errors;
/// Helper modules containing utility functions for Solana programs.
pub mod helpers;
