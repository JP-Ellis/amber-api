//! # Rust client for Amber Electric's API.

#![no_std]
#![expect(clippy::pub_use, reason = "Root API exports for convenience")]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod client;
mod error;
pub mod models;

#[cfg(feature = "std")]
pub use client::{Amber, AmberBuilder};
pub use error::{AmberError, Result};
