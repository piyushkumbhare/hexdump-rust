use super::args::Args;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, Read, Seek},
    os::windows::fs::MetadataExt,
};

/**
 * The LengthError is a custom error used when the
 * user tries to use the -s start offset option with a value
 * greater than or equal to the length of the file.
 *
 * Because `File::seek()` can lead to undefined behavior in this case,
 * we handle it ourselves with a custom error & detailed message.
 */
#[derive(Debug, Clone)]
struct LengthError {
    message: String,
}

impl Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for LengthError {}

/**
 * Entry point after the main function. This function is responsible for
 * checking that the file exists & calling the hexdump command with the correct
 * length. The main reason this is separate than main() is so we can test it.
 */
#[inline]
pub fn hexdump(args: Args) -> Result<String, Box<dyn Error>> {
    // Metadata lets us proactively check if the file exists AND grab it's size without opening it
    // This lets us avoid reading the ENTIRE file into a buffer in the case where `-n NUM` < buffer.len()
    let metadata = std::fs::metadata(&args.file)?;
    let file_length = metadata.file_size();

    if args.start >= file_length {
        return Err(Box::new(LengthError {
            message: format!(
                "Starting offset ({}) was larger than the file length ({})",
                args.start, file_length,
            ),
        }));
    }
    // Number of bytes we need to read
    let num: u64 = args.num.unwrap_or(file_length);

    // Open the file, seek to start pos, read to buffer
    let mut file = File::open(&args.file)?;
    file.seek(io::SeekFrom::Start(args.start))?;
    let mut file_limited = file.take(num);

    let mut buffer: Vec<u8> = vec![];
    file_limited.read_to_end(&mut buffer)?;

    // Call hexdump with the buffer we just filled
    let hex_string = hexdump_from_input(args, buffer)?;
    Ok(hex_string)
}

/**
 * Actually prints the hexdump. This is where the meat of the program lies.
 * the Args struct is required to pass in flag/option information from both CLI
 * execution and unit tests.
 */
#[inline]
pub fn hexdump_from_input(args: Args, buffer: Vec<u8>) -> io::Result<String> {
    // Instead of directly printing, we'll push to an output buffer and return this.
    // This allows us to assert the output of this function via testing.
    let mut output = String::new();

    // Length of a full line of bytes, used for padding in in -t option.
    let line_length = args.width * 2 + args.width / args.chunk_size;

    let mut offset: usize = 0;
    for line in buffer.chunks(args.width) {
        // If the -o option was not passed, print the offset
        if args.offset {
            output.push_str(format!("{:08x}  ", offset).as_str());
        }

        for chunk in line.chunks(args.chunk_size) {
            for byte in chunk {
                offset += 1;
                output.push_str(format!("{:02x}", byte).as_str());
            }
            output.push(' ');
        }

        // If the -t option was passed, print the translation
        if args.translate {
            // Add space padding 
            let current_line_length = line.len() * 2 + line.len() / args.chunk_size;
            let padding_amount = line_length - current_line_length;
            output.push_str(format!("{}", " ".repeat(padding_amount)).as_str());

            // ASCII Translation
            output.push_str("\t\t|");
            for chunk in line.chunks(args.chunk_size) {
                for byte in chunk {
                    // Translate each byte into a char, defaulting to '.' if not possible
                    // All whitespace becomes spaces
                    let ch = match *byte {
                        b'\n' | b'\t' | b'\r' => ' ',
                        32..=126 => char::from(*byte),
                        _ => '.',
                    };
                    output.push(ch);
                }
            }
            output.push('|');
        }
        output.push('\n');
    }
    Ok(output)
}
