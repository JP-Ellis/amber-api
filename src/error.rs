//! # Error types for the Amber Electric API client
//!
//! This module contains all error types and handling for the Amber API client.

/// Error types that can occur when using the Amber API client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AmberError {
    /// HTTP request error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] ureq::Error),
}

/// Result type for Amber API operations.
pub type Result<T> = core::result::Result<T, AmberError>;
