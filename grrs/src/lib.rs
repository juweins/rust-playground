use std::io::{BufRead, Write};
use std::path::PathBuf;
use log::{info, warn, error};

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
pub fn search_file(file: &PathBuf, pattern: &str, mut writer: impl std::io::Write) -> (u8) {
    info!("Searching for {} in file {}", pattern, file.display());

    let mut result = std::io::BufReader::new(std::fs::File::open(file).unwrap());
    let mut line = String::new();

    // Count the number of matches (for logging)
    let mut count = 0;

    // Read each line and check if it contains the pattern
    while result.read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern) {
            match write!(writer, "{}", line) {
                Ok(_) => (),
                Err(e) => error!("Error writing to stdout: {}", e),
            }
            count += 1;
        }
        line.clear();
    }
    count
    
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