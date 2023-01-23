use clap::Parser;
use std::io::Write;
use log::{info, error};

use grrs::{search_file, print_result, check_output, check_wildcards};


// Command line arguments
// - pattern: Pattern to search for
// - path: Path to the file to search
#[derive(Parser)]
struct Cli {

    /// Pattern to search for in file,
    /// e.g. amici
    pattern: String,

    /// Path to the file to search,
    /// e.g. example/example.txt
    path: std::path::PathBuf,

    /// Write the output to a file
    /// e.g. output.txt
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,

    /// Enable wildcards
    /// e.g. -w
    /// e.g. --wildcards
    #[clap(short, long)]
    wildcards: Option<bool>,

}


// Main function
// - Parses the command line arguments
// - Initializes the logger
// - Calls the search_file function with Writer set to stdout
fn main() {
    // Initialize
    let args = Cli::parse();
    env_logger::init();

    // Count for number of matches
    let mut count = 0;

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

    // check if output flag is set
    let save_output = check_output(&args.output);
    let wildcards = check_wildcards(&args.wildcards);

    if save_output {
        // create a file with specified name
        // TODO: How to handle highligting in file?
        let file = std::fs::File::create(&args.output.unwrap()).unwrap();
        count = search_file(&args.path, &args.pattern, wildcards, &file);
    } else {
        count = search_file(&args.path, &args.pattern, wildcards, &mut std::io::stdout());
    }

    // Print the result
    print_result(count, &args.pattern, &mut std::io::stdout());

}