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

    /// Translates the bytes to ASCII in-line whenever possible
    #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
    translate: bool,
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

    let line_length = args.width * 2 + args.width / args.chunk_size;

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
        
        // If the -t option was passed, print the translation
        if args.translate {
            // First we calculate & insert the necessary padding
            let current_line_length = line.len() * 2 + line.len() / args.chunk_size;
            let padding_amount = line_length - current_line_length;
            print!("{}", " ".repeat(padding_amount));

            // Next we print the |translation|
            // This is exactly like before, except we print a byte's ASCII rather than Hex
            print!("\t\t");
            print!("|");
            for chunk in line.chunks(args.chunk_size) {
                for byte in chunk {
                    // If the byte is NOT in the printable ASCII range (32 - 127), default to a ' '
                    let ch = match *byte {
                        32..=127 => char::from(*byte),
                        _ => ' ',
                    };
                    // Next, we revert all whitespace characters to be just ' '. This cleans up the output for the user
                    let ch = match ch {
                        '\n' => ' ',
                        '\t' => ' ',
                        '\r' => ' ',
                        _ => ch,
                    };
                    // Finally, we print the char
                    print!("{}", ch);
                }
            }
            print!("|");
        }

        println!()
    }
    exit(0);
}
