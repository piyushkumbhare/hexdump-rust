use std::{error::Error, process::exit};

use hexdump_rust::util::{args::Args, hexdump::*};

/*
By having main return a Result, we can take advantage of the ? operator to clean
up code AND have the process print detailed messages to the CLI on failure.
*/
fn main() -> Result<(), Box<dyn Error>> {
    /*
    Parse the command line arguments to build the args struct
    This struct will be passed through the program that contains flag & option information
    */
    let args = Args::new();

    // Call hexdump with the args & print result
    let output = hexdump(args)?;
    println!("{output}");

    exit(0);
}
