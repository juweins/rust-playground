use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use log::{info, warn, error};
use lipsum::lipsum;
use clap::Parser;
use colored::Colorize;


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
pub fn search_file(args: &Cli) -> (u8, HashMap<u8,String>) {

    let pattern = &args.pattern;
    let path = &args.path;
    let wildcard = &args.wildcards;
    let output = &args.output;

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
        line_number += 1;
        // search each word in the line for the pattern
        for word in line.split_whitespace() {
            // If wildcards are enabled, check if the word contains the pattern
            if wildcard.is_some() == true {
                if word.contains(pattern) == true {
                    // Highlight the pattern in the detected line
                    let res_line = line.replace(word, &word.red().to_string());

                    result.insert(line_number, res_line);
                    pattern_count += 1;
                }
            // If wildcards are not enabled, check if the word is equal to the pattern
            } else if wildcard.is_some() == false || wildcard.is_none() == true{
                if word.eq(pattern) {
                    // Highlight the pattern in the detected line
                    let res_line = line.replace(word, &word.red().to_string());

                    result.insert(line_number, res_line);
                    pattern_count += 1;
                }
            }
        }
        line.clear();
    }
    write_result(&result, pattern, writer);

    (pattern_count, result)
    
}

pub fn write_result(result: &HashMap<u8, String>, pattern: &str, mut writer: impl std::io::Write) {

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
