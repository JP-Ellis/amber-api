//! # Error types for the Amber Electric API client
//!
//! This module contains all error types and handling for the Amber API client.
//!
//! ## Rate Limit Errors
//!
//! The client handles rate limits automatically by default, but you may still
//! encounter rate limit errors if:
//!
//! - Automatic retries are disabled via `retry_on_rate_limit(false)`
//! - The maximum retry attempts are exhausted
//!
//! See [`AmberError::RateLimitExceeded`] and [`AmberError::RateLimitExhausted`]
//! for more details.

/// Error types that can occur when using the Amber API client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AmberError {
    /// HTTP request error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] ureq::Error),

    /// Rate limit exceeded. Contains the number of seconds to wait.
    ///
    /// This error is returned when the API rate limit is hit and automatic
    /// retries are disabled via `retry_on_rate_limit(false)`.
    #[error("Rate limit exceeded. Retry after {0} seconds")]
    RateLimitExceeded(u64),

    /// Rate limit exceeded and max retries exhausted.
    ///
    /// This error is returned when the API rate limit is hit and the maximum
    /// number of retry attempts has been exhausted. The `attempts` field shows
    /// how many retries were attempted, and `retry_after` shows the suggested
    /// wait time in seconds before trying again.
    #[error("Rate limit exceeded after {attempts} retry attempts. Last retry-after: {retry_after} seconds")]
    RateLimitExhausted {
        /// Number of retry attempts that were made.
        attempts: u32,
        /// Number of seconds to wait before retrying.
        retry_after: u64,
    },

    /// Unexpected HTTP status code.
    ///
    /// This error is returned when the API returns a non-2xx status code that
    /// is not specifically handled (e.g., not a rate limit error).
    #[error("HTTP {status}: {body}")]
    UnexpectedStatus {
        /// HTTP status code.
        status: u16,
        /// Response body (may be truncated or empty if unreadable).
        body: String,
    },
}

/// Result type for Amber API operations.
pub type Result<T> = core::result::Result<T, AmberError>;
