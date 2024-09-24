use std::error::Error;

use hexdump_rust::util::args::Args;
use hexdump_rust::util::hexdump::hexdump;

// Tests small input and performs exact comparison
#[test]
fn test_small_input() -> Result<(), Box<dyn Error>> {
    // Create args. Everything except file is defaulted.
    let args = Args {
        file: "tests/test-small-input.bin".to_string(),
        ..Default::default()
    };

    let output = hexdump(args)?;
    assert_eq!("00000000  3132 3334 \n", output);
    Ok(())
}

// Tests that offset values are correct
#[test]
fn test_offset() -> Result<(), Box<dyn Error>> {
    // Create args. Everything except file is defaulted.
    let args = Args {
        file: "tests/test-256B.bin".to_string(),
        ..Default::default()
    };

    let output = hexdump(args)?;

    /*
    To verify that the offset values are correct, we enumerate the lines and check if
    the offsets are all multiples of 16
    */
    for (line_num, line) in output.trim().split('\n').enumerate() {
        let offset = line.split("  ").collect::<Vec<&str>>()[0];
        // Format the current offset to 8-digit Hex with zeros as padding & compare
        assert_eq!(format!("{:08x}", line_num * 16), offset);
    }
    Ok(())
}

// Tests that the -n option works as intended
#[test]
fn test_custom_length() -> Result<(), Box<dyn Error>> {
    // Create args. Everything except file and num is defaulted.
    let args = Args {
        file: "tests/test-256B.bin".to_string(),
        num: Some(65),
        ..Default::default()
    };

    let output = hexdump(args)?;

    /*
    To verify that the -n flag works, we request only the first 65 bytes
    instead of the full 256. Then we just check to make sure that the last
    offset line reads 64 (00000040) and that the last line has only 1 byte.
    */

    if let Some(&last_line) = output.trim().split('\n').collect::<Vec<&str>>().last() {
        // Split on two spaces to separate offset & bytes
        let split_line: Vec<&str> = last_line.split("  ").collect();
        let offset = split_line[0];
        assert_eq!(format!("{:08x}", 64), offset);

        // 2 ASCII characters == 1 byte
        let last_byte = split_line[1];
        assert_eq!(2, last_byte.len());
    }

    Ok(())
}
