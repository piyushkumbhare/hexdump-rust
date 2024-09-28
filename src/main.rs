use std::{error::Error, process::exit};

mod args;
mod hexdump;
mod tests;

use args::Args;
use hexdump::Hexdump;

// By having main return a Result, we can take advantage of the ? operator to clean
// up code AND have the process print detailed messages to the CLI on failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command line arguments to build the args struct
    // This struct will be passed through the program as it contains flag & option information
    let args = Args::parse();

    let hexdump = Hexdump::new(args)?;

    // Iterate over the Hexdump parser, printing each line as we go
    for line in hexdump {
        // ? the line in case an error was thrown while reading
        let line = line?;
        println!("{line}");
    }

    exit(0);
}
