//! Error types for the application.
//!
//! This module uses `thiserror` to create a comprehensive enum
//! of all possible failure modes, from I/O errors to
//! format-specific parsing errors.

use std::path::PathBuf;
use thiserror::Error;

/// The main error enum for all operations in `transfor`.
#[derive(Debug, Error)]
pub enum TransforError {
    /// An error occurred during I/O operation (e.g., reading/writing a file).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred serializing or deserializing JSON data.
    #[error("JSON format error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error occurred deserializing TOML data.
    #[error("TOML read error: {0}")]
    Toml(#[from] toml::de::Error),

    /// An error occurred serializing TOML data.
    #[error("TOML write error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    /// An error occurred during a CSV operation (read or write).
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// An error for when the input file path is invalid Unicode.
    #[error("Invalid file path: {0:?}")]
    InvalidPath(PathBuf),

    /// An error for when the input format cannot be guessed and was not provided.
    #[error("Cannot determine input format for file: {0}")]
    UnknownInputFormat(String),

    /// An error for attempting to convert non-tabular data (e.g., a simple JSON
    /// object) into CSV, which requires an array of objects.
    #[error("Cannot convert non-tabular data to CSV. Input must be an array of objects.")]
    NonTabularForCsv,

    /// An error for when the input stream (e.g., stdin) is empty.
    #[error("Input stream was empty.")]
    EmptyInput,
}
