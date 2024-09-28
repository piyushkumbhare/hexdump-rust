use std::{process::exit, str::FromStr};

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
                    args.num = Some(parse_arg(arg, options.next()));
                }
                "-w" | "--width" => {
                    args.width = parse_arg(arg, options.next());
                }
                "-c" | "--chunk-size" => {
                    args.chunk_size = parse_arg(arg, options.next());
                }
                "-s" | "--start-offset" => {
                    args.start = parse_arg(arg, options.next());
                }
                "-o" | "--no-offset" => {
                    args.offset = false;
                }
                "-t" | "--translate" => {
                    args.translate = true;
                },
                "-h" | "--help" => {
                    println!(r#"
hexdump: A tool used to print/format the bytes of an input file.

Usage: hexdump [OPTIONS] <FILE>

    -n <NUM>                    Total number of bytes to read.
    -w --width <NUM>            Number of bytes to print per line. (Default: 16)
    -c --chunk-size <NUM>       Number of bytes to print per space-separated chunk. (Default: 2)
    -s --start-offset <NUM>     Starting offset to read file from. (Default: 0)
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

#[inline]
fn parse_arg<T: FromStr>(arg: &str, val: Option<&String>) -> T {
    val.unwrap_or_else(|| {
        eprintln!("Error: {arg} expects a number");
        exit(1);
    })
    .parse()
    .unwrap_or_else(|_| {
        eprintln!("Error: {arg} expects a number");
        exit(1);
    })
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
