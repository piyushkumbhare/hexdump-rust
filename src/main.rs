use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{self, Read},
    os::windows::fs::MetadataExt,
    process::exit,
};

/// A tool to print the contents of a file in hex.
#[derive(Debug, Parser)]
#[command(about)]
struct Args {
    /// File to print
    #[arg(required = true)]
    file: String,

    /// Total number of bytes to print
    #[arg(short)]
    num: Option<u64>,

    /// Number of bytes to print per line
    #[arg(short, long, default_value_t = 16)]
    width: usize,

    /// Don't print offset values
    #[arg(short, long = "no-offset", default_value_t = true, action = ArgAction::SetFalse)]
    offset: bool,

    /// Number of bytes per space-separated chunk
    #[arg(short, long = "chunk-size", default_value_t = 2)]
    chunk_size: usize,
}

fn main() -> io::Result<()> {
    // First we parse the command line arguments
    let args = Args::parse();

    // Next, we use metadata to proactively check if the file exists AND grab it's size without opening it
    // This lets us avoid reading the ENTIRE file into a buffer in the case where `-n NUM` < buffer.len()
    let metadata = std::fs::metadata(&args.file)?;

    // Then we get the number of bytes we need to read
    let num: u64 = args.num.unwrap_or(metadata.file_size());

    let file = File::open(&args.file)?;
    let mut buffer: Vec<u8> = vec![];

    // This limits the file to `len` bytes so we only read as much as we need to
    let mut file_limited = file.take(num);
    file_limited.read_to_end(&mut buffer)?;

    let mut offset: usize = 0;
    // Prints each line (default 16 bytes per line)
    for line in buffer.chunks(args.width) {
        
        // If the -o option was not passed, print the offset
        if args.offset {
            print!("{:08x}  ", offset);
        }

        // Prints the space-separated chunks (default 2 bytes per chunk)
        for chunk in line.chunks(args.chunk_size) {
            for byte in chunk {
                offset += 1;
                print!("{:02x}", byte);
            }
            print!(" ");
        }
        println!()
    }
    exit(0);
}
