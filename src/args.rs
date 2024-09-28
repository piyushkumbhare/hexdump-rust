use std::process::exit;

// use clap::{ArgAction, Parser};

/*
 * The Args struct is used to parse Command Line arguments easily
 * The documentation comments are parsed by the clap's macros to create
 * descriptions for each of the flags/options.
 */

/// A tool to print the contents of a file in hex.
// #[derive(Debug, Parser)]
// #[command(about)]
// pub struct Args {
//     /// File to print
//     #[arg(required = true)]
//     pub file: String,

//     /// Total number of bytes to print (will not read past EOF)
//     #[arg(short)]
//     pub num: Option<u64>,

//     /// Number of bytes to print per line
//     #[arg(short, long, default_value_t = 16)]
//     pub width: u64,

//     /// Don't print offset values
//     #[arg(short, long = "no-offset", default_value_t = true, action = ArgAction::SetFalse)]
//     pub offset: bool,

//     /// Number of bytes per space-separated chunk
//     #[arg(short, long = "chunk-size", default_value_t = 2)]
//     pub chunk_size: usize,

//     /// Translates the bytes to ASCII in-line whenever possible.
//     #[arg(short, long, default_value_t = false, action = ArgAction::SetTrue)]
//     pub translate: bool,

//     /// Starting offset to print from
//     #[arg(short, long = "start-offset", default_value_t = 0)]
//     pub start: u64,
// }

// // Creates struct by parsing CLI input. Used during user execution.
// impl Args {
//     pub fn new() -> Self {
//         Args::parse()
//     }
// }

#[derive(Debug)]
pub struct Args {
    pub file: String,
    pub num: Option<u64>,
    pub width: u64,
    pub chunk_size: usize,
    pub start: u64,
    pub translate: bool,
    pub offset: bool,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Args::default();

        let clargs: Vec<String> = std::env::args().collect();
        let mut options = clargs.get(1..).unwrap().iter();

        let mut found_file = false;

        while let Some(arg) = options.next() {
            match arg.as_str() {
                "-n" => {
                    args.num = Some(
                        options
                            .next()
                            .unwrap_or_else(|| {
                                eprintln!("Error: -n expects a number");
                                exit(1);
                            })
                            .parse()
                            .unwrap_or_else(|_| {
                                eprintln!("Error: -n expects a number");
                                exit(1);
                            }),
                    );
                }
                "-w" | "--width" => {
                    args.width = options
                        .next()
                        .unwrap_or_else(|| {
                            eprintln!("Error: -w expects a number");
                            exit(1);
                        })
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Error: -w expects a number");
                            exit(1);
                        })
                }
                "-c" | "--chunk-size" => {
                    args.chunk_size = options
                        .next()
                        .unwrap_or_else(|| {
                            eprintln!("Error: -c expects a number");
                            exit(1);
                        })
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Error: -c expects a number");
                            exit(1);
                        })
                }
                "-s" | "--start-offset" => {
                    args.start = options
                        .next()
                        .unwrap_or_else(|| {
                            eprintln!("Error: -s expects a number");
                            exit(1);
                        })
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Error: -s expects a number");
                            exit(1);
                        })
                }
                "-o" | "--no-offset" => {
                    args.offset = false;
                }
                "-t" | "--translate" => {
                    args.translate = true;
                },
                "-h" | "--help" => {
                    println!(r#"
Usage: hexdump [OPTIONS] <FILE>

    -n <NUM>                    Total number of bytes to read.
    -w --width <NUM>            Number of bytes to print per line.
    -c --chunk-size <NUM>       Number of bytes to print per space-separated chunk.
    -s --start-offset <NUM>     Starting offset to read file from.
    -t --translate              Enables in-line ASCII translation.
    -o --no-offset              Disables offset column.
    -h --help                   Prints this message.
                    "#);
                    exit(0);
                }
                filename => {
                    if !found_file {
                        found_file = true;
                        args.file = filename.to_string();
                    } else {
                        eprintln!("Unexpected value: {filename}");
                        eprintln!("Usage: hexdump [-n LEN] <FILE>");
                        eprintln!("Try hexdump --help for more info.");
                        exit(1);
                    }
                }
            }
        }
        if !found_file {
            eprintln!("Usage: hexdump [-n LEN] <FILE>");
            eprintln!("Try hexdump --help for more info.");
            exit(1);
        }
        args
    }
}

// Defines the default struct values
impl Default for Args {
    fn default() -> Self {
        Args {
            file: "example.txt".to_string(),
            num: None,
            start: 0,
            offset: true,
            chunk_size: 2,
            width: 16,
            translate: false,
        }
    }
}
