use thiserror::Error;

/// BaseError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum BaseError {
    /// Represents a failure to parse the input value
    #[error("Unable to parse input value")]
    ParseError { message: &'static str },

    /// Represents an invalid argument
    #[error("Invalid Arguments")]
    ArgError { message: &'static str },
}
