use clap::Parser;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use log::{info, warn};


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

    // Search for the pattern in the file
    let count = search_file(&args.path, &args.pattern, &mut std::io::stdout());

    // Print the result
    print_result(count, &args.pattern)
}

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
fn search_file(file: &PathBuf, pattern: &str, mut writer: impl std::io::Write) -> (u8) {
    info!("Searching for {} in file {}", pattern, file.display());

    let mut result = std::io::BufReader::new(std::fs::File::open(file).unwrap());
    let mut line = String::new();

    // Count the number of matches (for logging)
    let mut count = 0;

    // Read each line and check if it contains the pattern
    while result.read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern) {
            write!(writer, "{}", line);
            count += 1;
        }
        line.clear();
    }
    count
    // Check if there are any matches and print
    
}

fn print_result(count: u8, pattern: &str) {
    if count == 0 {
        warn!("No matches found for {}", pattern);
        println!("No matches found for {}", pattern);
    } else {
        info!("Found {} matches for {}", count, pattern);
        println!("Found {} matches for {}", count, pattern);
    }
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