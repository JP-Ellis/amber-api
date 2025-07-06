//! # Rust client for Amber Electric's API.

#![expect(clippy::pub_use, reason = "Root API exports for convenience")]

mod client;
mod error;
pub mod models;

pub use client::{Amber, AmberBuilder};
pub use error::{AmberError, Result};
