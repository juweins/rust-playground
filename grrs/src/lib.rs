mod cli;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use clap::*;
use log::{info, error};
use lipsum::lipsum;
use colored::Colorize;
use regex::RegexBuilder;
pub use cli::Cli;

// Takes a path and a pattern and searches for the pattern in the file
// - path: Path to the file to search
// - pattern: Pattern to search for
// - wildcards: If flag is set (true), the pattern will be handled as a wildcard
// TODO: Find a way to suppress word fragment highlighting in wildcards mode
pub fn search_file(args: &Cli) -> (u8, HashMap<u8,String>) {

    let pattern = &args.pattern;
    let path = &args.path;
    let wildcards = &args.wildcards;
    let output = &args.output;
    let ignore_case = &args.ignore_case;

    info!("Searching for {} in file {}", pattern, path.display());

    // Check if the output file flag is set
    // - Returns an Option<File> if set
    // - Returns None if not set
    let output_writer = check_output(output);

    // specify the writer depending on flag
    let writer = match output_writer {
        Some(_) => {
            let output_path = output.clone().unwrap();
            Box::new(File::create(output_path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(std::io::stdout()) as Box<dyn Write>,
    };

    
    let mut file = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut result: HashMap<u8, String> = HashMap::new();

    let mut line = String::new();


    // Count the number of matches (for logging)
    let mut pattern_count = 0;

    // Track the line number
    let mut line_number = 0;

    // Read each line and check if it contains the pattern
    // - If it does, write it to the writer and attach line number and increase count
    while file.read_line(&mut line).unwrap() > 0 {

        // preprocess the line by highlighting the pattern (case insensitive)
            let re = RegexBuilder::new(pattern)
            .case_insensitive(ignore_case.is_some())
            .build().unwrap();

        line_number += 1;
        // search each word in the line for the pattern
        for word in line.split_whitespace() {
            if re.is_match(word) {
                if wildcards.is_some() == true {
               
                    // Highlight the pattern in the detected line
                    let res_line = line.replace(word, &word.red().to_string());

                    result.insert(line_number, res_line);
                    pattern_count += 1;
                } else if wildcards.is_none() == true{
                        // Highlight the pattern in the detected line
                        let res_line = line.replace(word, &word.red().to_string());
    
                        result.insert(line_number, res_line);
                        pattern_count += 1;
                    }
            }

        }
        line.clear();
    }
    write_result(&result, writer);

    (pattern_count, result)
    
}

pub fn write_result(result: &HashMap<u8, String>, mut writer: impl std::io::Write) {

    for (key, value) in result { 
        match writeln!(writer, "{}: {}", key.to_string().red(), value) {
            Ok(_) => info!("Writing line {} to output", key),
            Err(e) => error!("Error writing line {} to output: {}", key, e),
        }
    }
    info!("Write result successful");
}

pub fn print_result(count: u8, pattern: &str, mut writer: impl std::io::Write) {

    let print_string = format!("Found {} matches for {}", count.to_string().red(), pattern.to_string().red());
    match writeln!(writer, "{}", print_string) {
        Ok(_) => info!("Writing result to output"),
        Err(e) => error!("Error writing result to output: {}", e),
    }
    info!("Found {} matches for {}", count, pattern);
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
// - REMOVED: print_result_match: Test if the print_result function returns the correct result
// - REMOVED: print_result_no_match: Test if the print_result function returns the correct result
// ----------------------
// - check_output_true: Test if the check_output function returns true
// - check_output_false: Test if the check_output function returns false
// ----------------------
// - check_wildcards_true: Test if the check_wildcards function returns true
// - check_wildcards_false: Test if the check_wildcards function returns false
// ----------------------
#[test]
fn search_file_match() {

    let args = Cli::parse_from(&["", "This", "examples/example.txt"]);
    let count = search_file(&args).0;
    assert_eq!(count, 1);
}

#[test]
fn search_file_no_match() {

    let args = Cli::parse_from(&["", "rustacean", "examples/example.txt"]);
    let count = search_file(&args).0;
    assert_eq!(count, 0);
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

#[test]
fn check_wildcards_true() {
    let wildcards = Some(true);
    assert_eq!(check_wildcards(&wildcards), true);
}

#[test]
fn check_wildcards_false() {
    let wildcards = None;
    assert_eq!(check_wildcards(&wildcards), false);
}

