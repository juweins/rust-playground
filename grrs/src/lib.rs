use std::fs::File;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use log::{info, warn, error};
use lipsum::lipsum;
use clap::Parser;


// Command line arguments
// - pattern: Pattern to search for
// - path: Path to the file to search
#[derive(Parser)]
pub struct Cli {

    /// Pattern to search for in file,
    /// e.g. amici
    pub pattern: String,

    /// Path to the file to search,
    /// e.g. example/example.txt
    pub path: std::path::PathBuf,

    /// Write the output to a file
    /// e.g. output.txt
    #[clap(short, long)]
    pub output: Option<std::path::PathBuf>,

    /// Enable wildcards
    /// e.g. -w
    /// e.g. --wildcards
    #[clap(short, long)]
    pub wildcards: Option<bool>,

}

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
// - wildcards: If flag is set (true), the pattern will be handled as a wildcard
// TODO: Find a way to suppress word fragment highlighting in wildcards mode
pub fn search_file(args: &Cli) -> u8 {

    let pattern = &args.pattern;
    let path = &args.path;
    let wildcard = &args.wildcards;
    let output = &args.output;

    info!("Searching for {} in file {}", pattern, path.display());


    let output_writer = check_output(output);
    // 
    let mut writer = match output_writer {
        Some(_) => {
            let output_path = output.clone().unwrap();
            Box::new(File::create(output_path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(std::io::stdout()) as Box<dyn Write>,
    };
    
    let mut result = std::io::BufReader::new(std::fs::File::open(path).unwrap());


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
            if wildcard.is_some() == true {
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
            } else if wildcard.is_some() == false || wildcard.is_none() == true{
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
// - If output is not set, set Writer to stdout
// return option of writer
pub fn check_output(output: &Option<PathBuf>) -> Option<File> {
    if output.is_some() == true{
        info!("Output file set to {}", output.as_ref().unwrap().display());
        let writer = std::fs::File::create(output.as_ref().unwrap()).unwrap();
        Option::Some(writer)
    } else {
        info!("Output file not set");
        Option::None
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

    let args = Cli::parse_from(&["", "This", "examples/example.txt"]);
    let count = search_file(&args);
    assert_eq!(count, 1);
}

#[test]
fn search_file_no_match() {

    let args = Cli::parse_from(&["", "rustacean", "examples/example.txt"]);
    let count = search_file(&args);
    assert_eq!(count, 0);
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
    let output = Some(PathBuf::from("examples/output.txt"));
    assert_eq!(check_output(&output).is_some(), true);
}

#[test]
fn check_output_false() {
    let output = None;
    assert_eq!(check_output(&output).is_none(), true);
}
