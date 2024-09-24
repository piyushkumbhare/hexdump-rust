use clap::{ArgAction, Parser};

/*
The Args struct is used to parse Command Line arguments easily
The documentation comments are parsed by the clap's derive macros to create
descriptions for each of the flags/options.
*/

/// A tool to print the contents of a file in hex.
#[derive(Debug, Parser)]
#[command(about)]
pub struct Args {
    /// File to print
    #[arg(required = true)]
    pub file: String,

    /// Total number of bytes to print
    #[arg(short)]
    pub num: Option<u64>,

    /// Number of bytes to print per line
    #[arg(short, long, default_value_t = 16)]
    pub width: usize,

    /// Don't print offset values
    #[arg(short, long = "no-offset", default_value_t = true, action = ArgAction::SetFalse)]
    pub offset: bool,

    /// Number of bytes per space-separated chunk
    #[arg(short, long = "chunk-size", default_value_t = 2)]
    pub chunk_size: usize,

    /// Translates the bytes to ASCII in-line whenever possible.
    /// Non-ASCII & whitespace will be visually converted to spaces.
    #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
    pub translate: bool,
}

impl Args {
    // Parses args based on CLI input arguments. Used by main program.
    pub fn new() -> Self {
        Args::parse()
    }
}

impl Default for Args {
    // Creates struct with default values. Used for testing purposes.
    fn default() -> Self {
        Args {
            file: "example.txt".to_string(),
            num: None,
            offset: true,
            chunk_size: 2,
            width: 16,
            translate: false,
        }
    }
}
