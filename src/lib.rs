//! # Transfor
//!
//! A high-performance CLI for data plumbin and format transformation.
//!
//! `transfor` reads data from `stdin` or a file, decodes it from CSV,
//! JSON, or TOML, and then serializes it to your chosen output
//! format.
//!
//! ## Core Modules
//!
//! - `cli`: Defines the command-line interface structure using `clap`.
//! - `error`: Contains the `thiserror` enum for all application errors.
//! - `formats`: Holds the core logic for reading and writing data formats.

pub mod cli;
pub mod error;
pub mod formats;
