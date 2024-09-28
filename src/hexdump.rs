use super::args::Args;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, Read, Seek},
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


pub struct Hexdump {
    args: Args,
    file: File,
    line_length: u64,
    total_bytes: u64,
    bytes_read: u64,
}

impl Hexdump {

    /// Opens the specified file and returns a `Hexdump` object.
    /// (The file will remain opened until the `Hexdump` object is dropped)
    /// 
    /// Will return an `Error` if the file is unable to be opened or 
    /// if the starting offset is larger than the size of the file.
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        // Metadata lets us proactively check if the file exists AND grab it's size without opening it
        // This lets us avoid reading the ENTIRE file into a buffer in the case where `-n NUM` < buffer.len()
        let metadata = std::fs::metadata(&args.file)?;
        let file_length;

        // Get the file size (libraries vary based on OS, so use #[cfg])
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            file_length = metadata.file_size();
        }

        #[cfg(target_os = "linux")]
        {
            use std::os::linux::fs::MetadataExt;
            file_length = metadata.st_size();
        }

        // Throw LengthError if start offset is out of bounds
        if args.start >= file_length {
            return Err(Box::new(LengthError {
                message: format!(
                    "Starting offset ({}) was larger than the file length ({})",
                    args.start, file_length,
                ),
            }));
        }
        // Number of bytes we need to read
        let total_bytes = args.num.unwrap_or(file_length) - args.start;

        // Open the file, seek to start pos, read to buffer
        let mut file = File::open(&args.file)?;
        file.seek(io::SeekFrom::Start(args.start))?;

        // Length of a full line of bytes, used for padding in in -t option.
        let line_length = args.width * 2 + args.width / args.chunk_size as u64;

        Ok(Self {
            args,
            file,
            line_length,
            total_bytes,
            bytes_read: 0,
        })
    }

    // Reads the next `width` bytes from the file. Errors if there were problems reading the file
    fn read_next(&mut self) -> io::Result<String> {
        // Vec that stores the line contents. This will be returned at the end
        // The size of this vec is limited by the -w --width option
        let mut buf;

        // Determines if we are on the last line (and if padding is needed)
        if self.total_bytes - self.bytes_read < self.args.width {
            buf = vec![0; (self.total_bytes - self.bytes_read) as usize];
        } else {
            buf = vec![0; self.args.width as usize];
        }

        self.file.read_exact(&mut buf)?;

        let mut output = String::new();

        // If the -o option was not passed, print the offset
        if self.args.offset {
            output.push_str(format!("{:08x}  ", self.bytes_read).as_str());
        }

        for chunk in buf.chunks(self.args.chunk_size) {
            for byte in chunk {
                self.bytes_read += 1;
                output.push_str(format!("{:02x}", byte).as_str());
            }
            output.push(' ');
        }

        // If the -t option was passed, print the translation
        if self.args.translate {
            // Add space padding
            let current_line_length = buf.len() * 2 + buf.len() / self.args.chunk_size;
            let padding_amount = self.line_length - current_line_length as u64;
            output.push_str(format!("{}", " ".repeat(padding_amount as usize)).as_str());

            // ASCII Translation
            output.push_str("\t\t|");
            for chunk in buf.chunks(self.args.chunk_size) {
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

        Ok(output)
    }
}

impl Iterator for Hexdump {
    type Item = io::Result<String>;

    /// Yields the next `width` bytes in the file.
    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes_read < self.total_bytes {
            Some(self.read_next())
        } else {
            None
        }
    }
}
