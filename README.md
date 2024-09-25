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

I decided not to copy the real `hexdump`'s features exactly and instead took the creative liberty to add ones that made sense and showcased Rust's power the best. Many of the original `hexdump`'s features are still present, but have just been generalized through different options/flags.

All added features mentioned above are working as intended and have tests to ensure their functionality.
## Examples

Here are examples of the non-obvious features on the same file used in the document:

### -w, --width \<NUM>
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

### -c, --chunk-size \<CHUNK_SIZE>
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

### -t, --translate
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
