use std::fmt::format;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::Arc;
use log::{info, warn, error};
use lipsum::lipsum;

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
pub fn search_file(file: &PathBuf, pattern: &str, mut writer: impl std::io::Write) -> u8 {
    info!("Searching for {} in file {}", pattern, file.display());

    let mut result = std::io::BufReader::new(std::fs::File::open(file).unwrap());
    let mut line = String::new();

    // Count the number of matches (for logging)
    let mut pattern_count = 0;

    // Track the line number
    let mut line_number = 0;

    // Read each line and check if it contains the pattern
    // - If it does, write it to the writer and attach line number and increase count
    while result.read_line(&mut line).unwrap() > 0 {
        line_number += 1;
        if line.contains(pattern) {
            
            // Highlight the matching line number and pattern (red)
            let line = line.replace(pattern, &format!("\x1b[1;31m{}\x1b[0m", pattern));
            match write!(writer, "{}: {}", &format!("\x1b[1;31m{}\x1b[0m", line_number), line) {
                Ok(_) => (),
                Err(e) => error!("Error writing to stdout: {}", e),
            }
            pattern_count += 1;
        }
        line.clear();
    }
    pattern_count
    
}

pub fn print_result(count: u8, pattern: &str, mut writer: impl std::io::Write) {
    if count == 0 {
        warn!("No matches found for {}", pattern);
        match writeln!(writer, "No matches found for {}", pattern) {
            Ok(_) => (),
            Err(e) => error!("Error writing to stdout: {}", e),
        }
    } else {
        info!("Found {} matches for {}", count, pattern);
        match writeln!(writer, "Found {} matches for {}", count, pattern) {
            Ok(_) => (),
            Err(e) => error!("Error writing to stdout: {}", e),
        }
    }
}

// Generate a sample file in example directory
// - This is used for testing
// - The file contains 1000 words of lorem ipsum
// - The file is overwritten if it already exists
pub fn generate_sample_file() {

    // Declare variables (Create directory and file if they don't exist)
    let mut file = std::fs::File::create("examples/example.txt").unwrap();
    let mut lipsum = lipsum(1000);

    // Add newlines to the text, to make it more accessible for test purposes
    lipsum = lipsum.replace(".", ".\n");
    lipsum = lipsum.replace("?", "?\n");
    lipsum = lipsum.replace("!", "!\n");

    file.write_all(lipsum.as_bytes()).unwrap();
}
    

// Tests
// - search_file_match: Test if the search_file function returns the correct result
// - search_file_no_match: Test if the search_file function returns the correct result
// - print_result_match: Test if the print_result function returns the correct result
// - print_result_no_match: Test if the print_result function returns the correct result
// ----------------------
// TODO: Find a way to make the search_file_match test more readable
#[test]
fn search_file_match() {

    let mut result: Vec<u8> = Vec::new();
    let count = search_file(&PathBuf::from("examples/example.txt"), "This", &mut result);
    assert_eq!(result, b"\x1b[1;31m1\x1b[0m: \x1b[1;31mThis\x1b[0m text has been generated using lipsum crate:\n");
    assert_eq!(count, 1);
}

#[test]
fn search_file_no_match() {

    let mut result: Vec<u8> = Vec::new();
    search_file(&PathBuf::from("examples/example.txt"), "rustacean", &mut result);
    assert_eq!(result, b"");
}

#[test]
fn print_result_match() {

    let mut result: Vec<u8> = Vec::new();
    print_result(1, "This", &mut result);
    assert_eq!(result, b"Found 1 matches for This\n");
}

#[test]
fn print_result_no_match() {

    let mut result: Vec<u8> = Vec::new();
    print_result(0, "rustacean", &mut result);
    assert_eq!(result, b"No matches found for rustacean\n");
}
