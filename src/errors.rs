//! Error types for changebase.
//!
//! This module defines all error types that can occur during base conversion.

use thiserror::Error;

/// Errors that can occur during base conversion operations.
#[derive(Error, Debug)]
pub enum BaseError {
    /// The input value could not be parsed in the specified (or detected) base.
    ///
    /// The `message` field contains a human-readable description of what
    /// digits are valid for the expected base.
    #[error("Unable to parse input value")]
    ParseError {
        /// Description of the parse error
        message: &'static str,
    },
}
