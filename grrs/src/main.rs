use clap::Parser;
use std::{io::BufRead, path::PathBuf};
use log::{info, warn};


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    env_logger::init();
    search_file(&args.path, &args.pattern);

}

fn search_file(file: &PathBuf, pattern: &str) {
    info!("Searching for {} in file {}", pattern, file.display());
    let mut result = std::io::BufReader::new(std::fs::File::open(file).unwrap());
    let mut line = String::new();
    let mut count = 0;

    while result.read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern) {
            print!("{}", line);
            count += 1;
        }
        line.clear();
    }

    // Check if there are any matches and print
    if count == 0 {
        warn!("No matches found for {}", pattern);
        println!("No matches found for {}", pattern);
    } else {
        info!("Found {} matches for {}", count, pattern);
        println!("Found {} matches for {}", count, pattern);
    }
    
}
