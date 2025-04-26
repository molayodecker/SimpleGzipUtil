# Simple Compressor

This project contains a Rust implementation of a simple file compression tool. The `main.rs` file serves as the entry point for the application, handling file input, compression, and output.

## Features

- Reads input files and compresses their content.
- Outputs compressed files with reduced size.
- Implements basic error handling for file operations.

## Usage

Run the application with the file path as an argument:

```bash
cargo run -- <fileInput><file_output>
```

## Dependencies

Ensure you have the required Rust libraries installed as specified in `Cargo.toml`.

## Notes

This is a basic implementation and may not handle all edge cases or large files efficiently.
