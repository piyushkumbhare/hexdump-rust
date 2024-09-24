use super::args::Args;
use std::{fs::File, io::{self, Read}, os::windows::fs::MetadataExt};


/*
Entry point after the main function. This function is responsible for
checking that the file exists & calling the hexdump command with the correct
length. The main reason this is separate than main() is so we can test it.
*/
#[inline]
pub fn hexdump(args: Args) -> io::Result<String> {
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

    // Call hexdump with the buffer we just filled
    let hex_string = hexdump_from_input(args, buffer)?;
    Ok(hex_string)
}


/*
Prints the hexdump given a buffer & the args. While args is mainly created via
clap's CLI parsing, it can be created manually. Thus by taking args as an argument,
we can pass info from main easily, as well as streamline unit testing.
*/
#[inline]
pub fn hexdump_from_input(args: Args, buffer: Vec<u8>) -> io::Result<String> {
    // Instead of directly printing, we'll push to an output buffer and return this.
    // This allows us to test this function.
    let mut output = String::new();

    // The length of a single line (not including offset bytes).
    // This is used to calculate padding when the -t flag is enabled
    let line_length = args.width * 2 + args.width / args.chunk_size;

    let mut offset: usize = 0;
    // Prints each line (default 16 bytes per line)
    for line in buffer.chunks(args.width) {
        // If the -o option was not passed, print the offset
        if args.offset {
            output.push_str(format!("{:08x}  ", offset).as_str());
        }

        // Prints the space-separated chunks (default 2 bytes per chunk)
        for chunk in line.chunks(args.chunk_size) {
            for byte in chunk {
                offset += 1;
                output.push_str(format!("{:02x}", byte).as_str());
            }
            output.push(' ');
        }

        // If the -t option was passed, print the translation
        if args.translate {
            // First we calculate & insert the necessary padding
            let current_line_length = line.len() * 2 + line.len() / args.chunk_size;
            let padding_amount = line_length - current_line_length;
            output.push_str(format!("{}", " ".repeat(padding_amount)).as_str());

            // Next we print the |translation|
            // This is exactly like before, except we print a byte's ASCII rather than Hex
            output.push_str("\t\t|");
            for chunk in line.chunks(args.chunk_size) {
                for byte in chunk {
                    // If the byte is NOT in the printable ASCII range (32 - 127), default to a ' '
                    let ch = match *byte {
                        32..=127 => char::from(*byte),
                        _ => '.',
                    };
                    // Next, we revert all whitespace characters to be just ' '. This cleans up the output for the user
                    let ch = match ch {
                        '\n' | '\t' | '\r' => ' ',
                        _ => ch,
                    };
                    // Finally, we print the char
                    output.push(ch);
                }
            }
            output.push('|');
        }
        output.push('\n');
    }
    Ok(output)
}
