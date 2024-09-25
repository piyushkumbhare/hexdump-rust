# Hexdump-Rust

Author: Piyush Kumbhare

This repository is my submission to Professor Porquet's screening test for Lab C (Educational OS in Rust). 

## About


The original hexdump functionality that was mentioned in the doc was simply to print the contents of a file in hex, with an optional `-n` flag to specify how many bytes to print. 

This functionality was achieved in the second commit of this repository, and can be seen by viewing that commit and its code or by clicking [here](https://github.com/piyushkumbhare/hexdump-rust/blob/9903da2fa5de0be99ad01463a7b11051df953f9f/src/main.rs)

The example given showcased the following Linux-style error message on an invalid program call:

```
$ hexdump
Usage: hexdump [-n LEN] FILE
```

I found that manually implementing a tool like `getopt` to replicate this format exactly would be too time consuming and honestly harmful in the long run. So instead of reinventing the wheel, I opted to use Rust's well known Clap crate.

Clap streamlines the entire process of parsing Command Line arguments through the power of macros. I really liked this approach because it not only made *coding* it easier, but it also becomes nearly trivial to add new flags and options (which one would likely do in the future). 

Plus, the message formatting of Clap is beautiful, as shown below:
```
$ hexdump_rust.exe
error: the following required arguments were not provided:
  <FILE>

Usage: hexdump_rust.exe <FILE>

For more information, try '--help'.
error: process didn't exit successfully: `hexdump_rust.exe` (exit code: 2)
```

```
$ hexdump_rust.exe --help
A tool to print the contents of a file in hex

Usage: hexdump_rust.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  File to print

Options:
  -n <NUM>              Total number of bytes to print (will not read past EOF)
  -h, --help            Print help
```

It also automatically catches and throws errors when an unknown flag or incorrect type is passed to the program, as shown below:

```
$ hexdump_rust.exe example.txt -n hi
error: invalid value 'hi' for '-n <NUM>': invalid digit found in string

For more information, try '--help'
```

## Improvements / Features

After completing the core `hexdump` functionality along with the `-n` option, I took it on as a challenge to implement more features as I saw fit. After all, I was dying to try out more of Clap's features. 

Here is an updated `--help` menu, which displays all the features I added.

```
$ hexdump_rust.exe --help
A tool to print the contents of a file in hex

Usage: hexdump_rust.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  File to print

Options:
  -n <NUM>                       Total number of bytes to print (will not read past EOF)
  -w, --width <WIDTH>            Number of bytes to print per line [default: 16]
  -o, --no-offset                Don't print offset values
  -c, --chunk-size <CHUNK_SIZE>  Number of bytes per space-separated chunk [default: 2]
  -t, --translate                Translates the bytes to ASCII in-line whenever possible
  -s, --start-offset <START>     Offset to start printing from [default: 0]
  -h, --help                     Print help
```

(I decided not to add the real `hexdump`'s features and instead added ones that made sense and showcased Rust's power the best)

All added features mentioned above are working as intended, but not thoroughly tested yet.

## Testing

This project can be tested via Cargo's built-in testing tool.

The current tests only apply to the core `hexdump` and `-n` functionality, so the others are not covered yet.

All tests are located within `tests/integration_test.rs` and linted with the `#[test]` macro. To run all tests, simply run `cargo test` and a detailed summary of the results will be printed to the screen.

## Final Thoughts

I loved working on this project, as it tested my knowledge of Rust as well as put me into the mindset of writing "kernel level" code. Writing my own Error types and ensuring that the program should never fail unexpectedly was a fun challenge to take on.

I may continue working on this project even after Professor Porquet's lab applications close, so if you have any suggestions on features I should add or coding conventions, please feel free to let me know!
