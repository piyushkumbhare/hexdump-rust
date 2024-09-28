# Hexdump-Rust

Author: Piyush Kumbhare

This repository is my submission to Professor Porquet's screening test for Lab C (Educational OS in Rust). 

## About


The original hexdump functionality that was mentioned in the doc was simply to print the contents of a file in hex, with an optional `-n` flag to specify how many bytes to print. 

This functionality was achieved in the second commit of this repository, and can be seen by viewing that commit and its code or by clicking [here](https://github.com/piyushkumbhare/hexdump-rust/blob/9903da2fa5de0be99ad01463a7b11051df953f9f/src/main.rs).

When it comes to parsing arguments, I originally used `Clap`. However, while it drastically simplifies the code and streamlines the process of parsing CLI arguments, it was brought to my attention that kernel-level code should *avoid* using non-`std` modules. Hence, I reverted the program back to using a manual argument parser.

## Improvements / Features

After completing the core `hexdump` functionality along with the `-n` option, I took it on as a challenge to implement more features as I saw fit. 

Here is an updated `--help` menu, which displays all the features I added.

```
$ hexdump_rust.exe --help

hexdump: A tool used to print/format the bytes of an input file.

Usage: hexdump [OPTIONS] <FILE>

    -n <NUM>                    Total number of bytes to read.
    -w --width <NUM>            Number of bytes to print per line. (Default: 16)
    -c --chunk-size <NUM>       Number of bytes to print per space-separated chunk. (Default: 2)
    -s --start-offset <NUM>     Starting offset to read file from. (Default: 0)
    -t --translate              Enables in-line ASCII translation.
    -o --no-offset              Disables offset column.
    -h --help                   Prints this message.
```

I decided not to copy the real `hexdump`'s features exactly and instead took the creative liberty to add ones that made sense and showcased Rust's power the best. Many of the original `hexdump`'s features are still present, but have just been generalized through different options/flags.

All added features mentioned above are working as intended and have tests to ensure their functionality.

When it comes to error handling, we can use a nifty trick of having `main()` returning a `Result` type. This way, Rust will automatically print the `Err` type in the case of any failure:
```
$ hexdump_rust.exe not-a-file
Error: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
error: process didn't exit successfully: `hexdump_rust.exe not-a-file` (exit code: 1)
```

```
$ hexdump_rust.exe example.bin -s 257
Error: LengthError { message: "Starting offset (257) was larger than the file length (256)" }   
error: process didn't exit successfully: `hexdump_rust.exe example.bin -s 257` (exit code: 1)
```

An added benefit of this approach is to allow us to utilize the `?` operator on any `Result` type present within the code. This drastically cleans up the logic and makes writing new code *much* easier.

## Examples

Here are examples of the non-obvious features on the same file used in the document:

### `-w, --width <NUM>`
```
$ hexdump_rust.exe hexdump -n 256 -w 8
00000000  7f45 4c46 0201 0100 
00000008  0000 0000 0000 0000
00000010  0200 f300 0100 0000
00000018  b606 0100 0000 0000
00000020  4000 0000 0000 0000
00000028  785c 0000 0000 0000
00000030  0100 0000 4000 3800
00000038  0400 4000 1100 1000
00000040  0300 0070 0400 0000
00000048  2330 0000 0000 0000
00000050  0000 0000 0000 0000
00000058  0000 0000 0000 0000
00000060  4a00 0000 0000 0000
00000068  0000 0000 0000 0000
00000070  0100 0000 0000 0000
00000078  0100 0000 0500 0000
00000080  0010 0000 0000 0000
00000088  0000 0100 0000 0000
00000090  0000 0100 0000 0000
00000098  9f10 0000 0000 0000
000000a0  9f10 0000 0000 0000
000000a8  0010 0000 0000 0000
000000b0  0100 0000 0600 0000
000000b8  0030 0000 0000 0000
000000c0  0020 0100 0000 0000
000000c8  0020 0100 0000 0000
000000d0  1100 0000 0000 0000
000000d8  1802 0000 0000 0000
000000e0  0010 0000 0000 0000
000000e8  51e5 7464 0600 0000
000000f0  0000 0000 0000 0000
000000f8  0000 0000 0000 0000
```

### `-c, --chunk-size <NUM>`
```
$ hexdump_rust.exe hexdump -n 256 -c 1
00000000  7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00 
00000010  02 00 f3 00 01 00 00 00 b6 06 01 00 00 00 00 00
00000020  40 00 00 00 00 00 00 00 78 5c 00 00 00 00 00 00
00000030  01 00 00 00 40 00 38 00 04 00 40 00 11 00 10 00
00000040  03 00 00 70 04 00 00 00 23 30 00 00 00 00 00 00
00000050  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00000060  4a 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00000070  01 00 00 00 00 00 00 00 01 00 00 00 05 00 00 00
00000080  00 10 00 00 00 00 00 00 00 00 01 00 00 00 00 00
00000090  00 00 01 00 00 00 00 00 9f 10 00 00 00 00 00 00
000000a0  9f 10 00 00 00 00 00 00 00 10 00 00 00 00 00 00
000000b0  01 00 00 00 06 00 00 00 00 30 00 00 00 00 00 00
000000c0  00 20 01 00 00 00 00 00 00 20 01 00 00 00 00 00
000000d0  11 00 00 00 00 00 00 00 18 02 00 00 00 00 00 00
000000e0  00 10 00 00 00 00 00 00 51 e5 74 64 06 00 00 00
000000f0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

### `-t, --translate`
```
$ hexdump_rust.exe hexdump -n 256 -t
00000000  7f45 4c46 0201 0100 0000 0000 0000 0000               |.ELF............|     
00000010  0200 f300 0100 0000 b606 0100 0000 0000               |................|     
00000020  4000 0000 0000 0000 785c 0000 0000 0000               |@.......x\......|     
00000030  0100 0000 4000 3800 0400 4000 1100 1000               |....@.8...@.....|     
00000040  0300 0070 0400 0000 2330 0000 0000 0000               |...p....#0......|     
00000050  0000 0000 0000 0000 0000 0000 0000 0000               |................|     
00000060  4a00 0000 0000 0000 0000 0000 0000 0000               |J...............|     
00000070  0100 0000 0000 0000 0100 0000 0500 0000               |................|     
00000080  0010 0000 0000 0000 0000 0100 0000 0000               |................|     
00000090  0000 0100 0000 0000 9f10 0000 0000 0000               |................|     
000000a0  9f10 0000 0000 0000 0010 0000 0000 0000               |................|     
000000b0  0100 0000 0600 0000 0030 0000 0000 0000               |.........0......|     
000000c0  0020 0100 0000 0000 0020 0100 0000 0000               |. ....... ......|     
000000d0  1100 0000 0000 0000 1802 0000 0000 0000               |................|     
000000e0  0010 0000 0000 0000 51e5 7464 0600 0000               |........Q.td....|     
000000f0  0000 0000 0000 0000 0000 0000 0000 0000               |................|
```

(It's important to note that on some architectures, the `hexdump` command reverses each "chunk" due to some processors using the little-endian convention for 16-bit words. My implementation of the program does not do this, and instead prints all bytes in order)

## Testing

This project can be tested via Cargo's built-in testing tool.

All tests are located within `tests/integration_test.rs` and linted with the `#[test]` macro. To run all tests, simply run `cargo test` and a detailed summary of the results will be printed to the screen. All tests use `.bin` or `.txt` files located within the `tests/` directory, so please ensure you pull these before running the tests.

## Final Thoughts

I loved working on this project, as it tested my knowledge of Rust as well as put me into the mindset of writing "kernel level" code. Writing my own Error types and ensuring that the program should never fail unexpectedly was a fun challenge to take on.

I may continue working on this project even after Professor Porquet's lab applications close, so if you have any suggestions on features I should add or coding conventions, please feel free to let me know!
