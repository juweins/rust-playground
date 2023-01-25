use clap::*;
use std::io::Write;
use log::{info, error};

use grrs::{Cli, search_file, print_result};


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

    let result = search_file(&args);

    // Print the result
    print_result(result.0, &args.pattern, std::io::stdout());

}