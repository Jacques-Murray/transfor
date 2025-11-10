//! Main binary entry point for the `transfor` CLI.
//!
//! This file is reponsible for:
//! - Parsing command-line arguments using `clap`.
//! - Setting up input (stdin or file) and output (stdout or file) streams.
//! - Guessing the input format if not provided.
//! - Calling the core `read_data` and `write_data` functions.
//! - Handling and printing any errors to stderr.

use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

use transfor::cli::{Cli, Format};
use transfor::error::TransforError;
use transfor::formats::{read_data, write_data};

/// Main function: executes the CLI logic.
fn main() {
    // Run the main logic and exit with an error code if it fails.
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// The primary logic loop of the application.
fn run() -> Result<(), TransforError> {
    let cli = Cli::parse();

    // 1. Determine input format
    let input_format = guess_input_format(cli.input_file.as_deref(), cli.input_format)?;

    // 2. Set up input reader
    let mut reader = get_reader(cli.input_file)?;

    // 3. Set up output writer
    let mut writer = get_writer(cli.output_file)?;

    // 4. Run the transformation
    // Read data -> Convert to intermediate Value
    let data = read_data(&mut reader, input_format)?;

    // Write data -> Serialize from intermediate Value
    write_data(&mut writer, &data, cli.output_format)?;

    Ok(())
}

/// Gets a boxed `Read` trait object from stdin or a file.
fn get_reader(input: Option<PathBuf>) -> Result<Box<dyn Read>, TransforError> {
    match input {
        Some(path) => {
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        }
        None => Ok(Box::new(BufReader::new(io::stdin()))),
    }
}

/// Gets a boxed `Write` trait object to stdout or a file.
fn get_writer(output: Option<PathBuf>) -> Result<Box<dyn Write>, TransforError> {
    match output {
        Some(path) => {
            let file = File::create(path)?;
            Ok(Box::new(BufWriter::new(file)))
        }
        None => Ok(Box::new(BufWriter::new(io::stdout()))),
    }
}

/// Guesses the input format from the file extension if not provided.
fn guess_input_format(
    input: Option<&Path>,
    format: Option<Format>,
) -> Result<Format, TransforError> {
    // If format is provided, use it.
    if let Some(f) = format {
        return Ok(f);
    }

    // If no format and no file, we can't guess.
    let path = match input {
        Some(p) => p,
        None => return Err(TransforError::UnknownInputFormat("stdin".to_string())),
    };

    // Guess from file extension.
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => Ok(Format::Json),
        Some("toml") => Ok(Format::Toml),
        Some("csv") => Ok(Format::Csv),
        _ => Err(TransforError::UnknownInputFormat(
            path.display().to_string(),
        )),
    }
}
