//! Defines the command-line interface for the application.
//!
//! Uses `clap` to parse arguments, including input/output formats
//! and file paths.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// A high-performance CLI for data plumbing and format transformation.
///
/// Reads from stdin or a file, decodes CSV, JSON, or TOML,
/// and writes the result to stdout or a file in your chosen format.
///
/// Example:
///   cat data.csv | transfor -i csv -o json
///
/// Example:
///   transfor -o json data.toml
#[derive(Parser, Debug)]
#[command(version, author = "Jacques Murray", about)]
pub struct Cli {
    /// The input format. If absent, will try to guess from file extension.
    /// Required if reading from stdin.
    #[arg(short = 'i', long)]
    pub input_format: Option<Format>,

    /// The desired output format.
    #[arg(short = 'o', long)]
    pub output_format: Format,

    /// The input file to read. If absent, reads from stdin.
    #[arg()]
    pub input_file: Option<PathBuf>,

    /// The output file to write to. If absent, writes to stdout.
    #[arg(short = 'w', long)]
    pub output_file: Option<PathBuf>,
}

/// The set of supported data formats.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// JavaScript Object Notation
    Json,
    /// TOML's Obvious, Minimal Language
    Toml,
    /// Comma-Separated Values
    Csv,
}
