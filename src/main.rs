use clap::Parser;
use std::{fs::File, io::{self, Read}, os::windows::fs::MetadataExt, process::exit};

/// A tool to print the contents of a file in hex.
#[derive(Debug, Parser)]
#[command(about)]
struct Args {
    /// File to print
    #[arg(required = true)]
    file: String,

    /// Number of bytes to print
    #[arg(short)]
    num: Option<u64>,
}

fn main() {
    // First we parse the command line arguments
    let args = Args::parse();

    // Check the metadata of the file. If any errors are thrown, report them to stderr and exit with code 1
    // We use std::fs::metadata instead of buffer.len() to avoid reading the whole file in the case where `-n NUM` < buffer.len() 
    match std::fs::metadata(&args.file) {
        Ok(md) => {
            // If the file exists and no errors are thrown, we can now print the hexdump of the file

            // The length to be used is provided by the -n flag, but defaulted to the size of the file
            let len = args.num.unwrap_or(md.file_size());
            
            // Call the print_hex funciton. If any error is encountered, print it to stderr and exit with code 1
            if let Err(e) = print_hex(&args.file, len) {
                eprintln!("Ran into the following file error: {e}");
                exit(1);                    
            }
            // On success, exit with code 0
            exit(0);
        },
        Err(e) => {
            eprintln!("Ran into the following file error: {e}");
            exit(1);
        },
    }
}

/// This function will print the first `len` bytes of the file at `path`
/// 
/// All inner errors are propatgated out to the parent function
fn print_hex(path: &str, len: u64) -> io::Result<()> {

    // Open the file and create an empty buffer
    let file = File::open(path)?;
    let mut buffer: &mut Vec<u8> = &mut vec![];

    // This limits the file to `len` bytes so we only read as much as we need to
    let mut file_limited = file.take(len);
    file_limited.read_to_end(&mut buffer)?;
    let mut offset: usize = 0;
    for chunk in buffer.chunks(16) {
        print!("{:08x}  ", offset);
        for two_bytes in chunk.chunks(1) {
            print!("{:02x}", two_bytes[0]);
            if let Some(second_byte) = two_bytes.get(1) {
                print!("{:02x}", second_byte);
            }
            print!(" ");
        }
        offset += 16;
        println!()
    }
    Ok(())
} 