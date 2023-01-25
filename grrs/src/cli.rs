// Command line arguments
// - pattern: Pattern to search for
// - path: Path to the file to search
#[derive(clap::Parser)]
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