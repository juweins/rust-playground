use std::io::{BufRead, Write};
use std::path::PathBuf;
use log::{info, warn, error};
use lipsum::lipsum;

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
// - wildcards: If flag is set (true), the pattern will be handled as a wildcard
// TODO: Find a way to suppress word fragment highlighting in wildcards mode
pub fn search_file(file: &PathBuf, pattern: &str,wildcards: bool, mut writer: impl std::io::Write) -> u8 {
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
        // search each word in the line for the pattern
        for word in line.split_whitespace() {
            // If wildcards are enabled, check if the word contains the pattern
            if wildcards == true {
                if word.contains(pattern) == true {
                    pattern_count += 1;
                    // Highlight the matching line number and pattern (red)
                    let line = line.replace(pattern, &format!("\x1b[1;31m{}\x1b[0m", pattern));
                    match writeln!(writer, "{}: {}", &format!("\x1b[1;31m{}\x1b[0m", line_number), line) {
                        Ok(_) => (),
                        Err(e) => error!("Error writing: {}", e),
                    }
                }
            // If wildcards are not enabled, check if the word is equal to the pattern
            } else if wildcards == false{
                if word.eq(pattern) {
                    // TODO: Suppress subsequent word fragment highlighting in match
                    let line = line.replace(word, &format!("\x1b[1;31m{}\x1b[0m", word));
                    match writeln!(writer, "{}: {}", &format!("\x1b[1;31m{}\x1b[0m", line_number), line) {
                        Ok(_) => (),
                        Err(e) => error!("Error writing: {}", e),
                    }
                    pattern_count += 1;
                }
            }
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

// check if output is set
// - If output is set, set Writer to output file
pub fn check_output(output: &Option<PathBuf>) -> bool {
    if let Some(output) = output {
        info!("Output file set to {}", output.display());
        true
    } else {
        info!("Output file not set");
        false
    }
}

// check if wildcards are set
// - If wildcards are set, set wildcards to true
pub fn check_wildcards(wildcards: &Option<bool>) -> bool {
    if let Some(wildcards) = wildcards {
        info!("Wildcards set to {}", wildcards);
        *wildcards
    } else {
        info!("Wildcards not set");
        false
    }
}
    

// Tests
// - search_file_match: Test if the search_file function returns the correct result
// - search_file_no_match: Test if the search_file function returns the correct result
// ----------------------
// - print_result_match: Test if the print_result function returns the correct result
// - print_result_no_match: Test if the print_result function returns the correct result
// ----------------------
// - check_output_true: Test if the check_output function returns true
// - check_output_false: Test if the check_output function returns false
// ----------------------
// TODO: Find a way to make the search_file_match test more readable
#[test]
fn search_file_match() {

    let mut result: Vec<u8> = Vec::new();
    let count = search_file(&PathBuf::from("examples/example.txt"), "This",false, &mut result);
    assert_eq!(result, b"\x1b[1;31m1\x1b[0m: \x1b[1;31mThis\x1b[0m text has been generated using lipsum crate:\n");
    assert_eq!(count, 1);
}

#[test]
fn search_file_no_match() {

    let mut result: Vec<u8> = Vec::new();
    search_file(&PathBuf::from("examples/example.txt"), "rustacean", false, &mut result);
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

#[test]
fn check_output_true() {
    assert_eq!(check_output(&Some(PathBuf::from("examples/example.txt"))), true);
}

#[test]
fn check_output_false() {
    assert_eq!(check_output(&None), false);
}
