#[warn(unused_imports)]

use clap::Parser;
use std::{path::PathBuf, sync::Arc, io::{stderr, Write}, fmt::Error};
use log::{info, warn, error};

use grrs::{search_file, print_result};


// Command line arguments
// - pattern: Pattern to search for
// - path: Path to the file to search
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// Main function
// - Parses the command line arguments
// - Initializes the logger
// - Calls the search_file function with Writer set to stdout
fn main() {
    // Initialize
    let args = Cli::parse();
    env_logger::init();

    // check pattern is empty
    // - If pattern is empty, raise error with message
    match args.pattern.is_empty() {
        true => {
            error!("No pattern provided!");
            std::io::stderr().write_all(b"No pattern provided!").unwrap();
    },
        false => {
            info!("Pattern provided: {}", args.pattern);
        }
    }

    let count = search_file(&args.path, &args.pattern, &mut std::io::stdout());

    // Print the result
    print_result(count, &args.pattern, &mut std::io::stdout())
}

// Tests
// - search_file_match: Test if the search_file function returns the correct result
// - search_file_no_match: Test if the search_file function returns the correct result
#[test]
fn search_file_match() {

    let mut result: Vec<u8> = Vec::new();
    search_file(&PathBuf::from("examples/example.txt"), "Hello", &mut result);
    assert_eq!(result, b"Hello\n");
}

#[test]
fn search_file_no_match() {

    let mut result: Vec<u8> = Vec::new();
    search_file(&PathBuf::from("examples/example.txt"), "Servus", &mut result);
    assert_eq!(result, b"");
}