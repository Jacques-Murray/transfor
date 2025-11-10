# Transfor

A high-performance CLI for data plumbing and format transformation.

`transfor` reads data from `stdin` or a file, decodes it from CSV, JSON, or TOML, and then serializes it to your chosen output format.

## Features

- 🚀 **High Performance**: Built in Rust for speed and efficiency
- 🔄 **Multiple Formats**: Convert between JSON, TOML, and CSV seamlessly
- 📊 **Flexible I/O**: Read from files or stdin, write to files or stdout
- 🎯 **Smart Format Detection**: Automatically detect input format from file extensions
- 💪 **Type Safe**: Comprehensive error handling with descriptive messages

## Supported Formats

- **JSON** (JavaScript Object Notation)
- **TOML** (TOML's Obvious, Minimal Language)
- **CSV** (Comma-Separated Values)

## Installation

### From Source

```bash
git clone https://github.com/Jacques-Murray/transfor.git
cd transfor
cargo build --release
```

The binary will be available at `target/release/transfor`.

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Syntax

```bash
transfor [OPTIONS] --output-format <OUTPUT_FORMAT> [INPUT_FILE]
```

### Options

- `-i, --input-format <FORMAT>`: The input format (json, toml, or csv). If not provided, will be guessed from the file extension. Required when reading from stdin.
- `-o, --output-format <FORMAT>`: The desired output format (json, toml, or csv). **Required**.
- `-w, --output-file <FILE>`: The output file to write to. If absent, writes to stdout.
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Examples

#### Convert CSV to JSON

```bash
transfor -o json data.csv
```

#### Convert JSON to TOML

```bash
transfor -o toml data.json
```

#### Convert TOML to CSV (with explicit input format)

```bash
transfor -i toml -o csv data.toml
```

#### Read from stdin and write to file

```bash
cat data.csv | transfor -i csv -o json -w output.json
```

#### Chain transformations

```bash
cat data.json | transfor -i json -o csv | transfor -i csv -o toml
```

### Sample Data Files

The `data/` directory contains sample files for testing:

- `data/data.csv` - Sample CSV file with user data
- `data/data.json` - Sample JSON file with project metadata
- `data/data.toml` - Sample TOML file with database configuration

### Format-Specific Notes

#### CSV Output

When converting to CSV, the input data must be:
- An array of objects (tabular data)
- All objects should have consistent fields

Example valid JSON for CSV conversion:
```json
[
  {"id": "1", "name": "Alice", "city": "New York"},
  {"id": "2", "name": "Bob", "city": "London"}
]
```

#### TOML Output

When converting to TOML:
- Arrays at the root level will be wrapped in a `data` key
- Complex nested structures are supported

#### CSV Input

CSV data is treated as:
- Headers from the first row become object keys
- All values are initially treated as strings
- Output is an array of objects

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the CLI in Development

```bash
cargo run -- [OPTIONS] [INPUT_FILE]
```

For example:
```bash
cargo run -- -o json data/data.csv
```

## Error Handling

`transfor` provides clear error messages for common issues:

- **Unknown Input Format**: When the input format cannot be determined
- **Non-Tabular Data to CSV**: When attempting to convert non-array data to CSV
- **Empty Input**: When the input stream or file is empty
- **I/O Errors**: File not found, permission denied, etc.
- **Format Parsing Errors**: Invalid JSON, TOML, or CSV syntax

## Architecture

The project is organized into several modules:

- **`cli`**: Command-line interface definitions using `clap`
- **`error`**: Error types using `thiserror`
- **`formats`**: Core transformation logic for reading and writing data
- **`main`**: Binary entry point and I/O handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [Cargo.toml](Cargo.toml) file for details.

## Author

Jacques Murray (jacquesmmurray@gmail.com)

## Repository

https://github.com/Jacques-Murray/transfor
