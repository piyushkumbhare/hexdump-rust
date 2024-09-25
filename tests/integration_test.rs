use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::os::windows::fs::MetadataExt;

use hexdump_rust::util::args::Args;
use hexdump_rust::util::hexdump::hexdump;

/**
 * Test: Core functionality
 * Brute force checks a small file to see if the bytes printed
 * match that of the contents
 */
#[test]
fn test_small_input() -> Result<(), Box<dyn Error>> {
    // All defaults
    let args = Args {
        file: "tests/test-small-input.bin".to_string(),
        ..Default::default()
    };

    let output = hexdump(args)?;
    assert_eq!("00000000  3132 3334 \n", output);
    Ok(())
}

/**
 * Test: Core functionality
 * To verify that the offset values are correct, enumerate the
 * lines and check if the offsets are all multiples of 16
 */
#[test]
fn test_offset() -> Result<(), Box<dyn Error>> {
    // All defaults
    let args = Args {
        file: "tests/test-256B.bin".to_string(),
        ..Default::default()
    };

    let output = hexdump(args)?;

    for (line_num, line) in output.trim().split('\n').enumerate() {
        let offset = line.split("  ").collect::<Vec<&str>>()[0];
        // Format the current offset to 8-digit Hex with zeros as padding & compare
        assert_eq!(format!("{:08x}", line_num * 16), offset);
    }
    Ok(())
}

/**
 * Test: -n flag
 * Request 65 out of the full 256 bytes. Then just check to make sure that the last
 * offset line reads 64 (0x00000040) and that the last line has only 1 byte.
 */
#[test]
fn test_custom_length() -> Result<(), Box<dyn Error>> {
    // -n 65
    let args = Args {
        file: "tests/test-256B.bin".to_string(),
        num: Some(65),
        ..Default::default()
    };

    let output = hexdump(args)?;

    if let Some(&last_line) = output.trim().split('\n').collect::<Vec<&str>>().last() {
        let split_line: Vec<&str> = last_line.split("  ").collect();
        let offset = split_line[0];
        assert_eq!(format!("{:08x}", 64), offset);

        let last_byte = split_line[1];
        // 2 ASCII characters == 1 byte
        assert_eq!(2, last_byte.len());
    }

    Ok(())
}

/*
 * Below are tests that test custom added functionality.
 */

/**
 * Test: -t flag
 * Hexdump a text file and verify the string built by the
 * translation matches that of the text file.
 */
#[test]
fn test_translate() -> Result<(), Box<dyn Error>> {
    // -t
    let args = Args {
        file: "tests/test-ASCII.txt".to_string(),
        translate: true,
        ..Default::default()
    };

    let output = hexdump(args)?;

    // Find all characters located within | | and collect them into a string.
    let translated_string: String = output
        .trim()
        .split('\n')
        .map(|line| line.split('|').collect::<Vec<&str>>()[1])
        .collect();

    let mut file = File::open("tests/test-ASCII.txt")?;
    let mut actual_string = String::new();
    file.read_to_string(&mut actual_string)?;

    assert_eq!(actual_string, translated_string);

    Ok(())
}

/**
 * Test: -o flag
 * Ensure that offset is not present in output.
 * This can be done via brute force with small input.
 */
#[test]
fn test_no_offset() -> Result<(), Box<dyn Error>> {
    // -o
    let args = Args {
        file: "tests/test-small-input.bin".to_string(),
        offset: false,
        ..Default::default()
    };

    let output = hexdump(args)?;
    assert_eq!("3132 3334 \n", output);
    Ok(())
}

/**
 * Test: -c flag
 * Ensure that there are chunks of length 1 byte each.
 * Also check that total number of bytes matches length of file.
 * (Disables offset for simplicity)
 */
#[test]
fn test_chunk_size() -> Result<(), Box<dyn Error>> {
    // -o -c 1
    let args = Args {
        file: "tests/test-ASCII.txt".to_string(),
        chunk_size: 1,
        offset: false,
        ..Default::default()
    };

    let output = hexdump(args)?;

    let file_len = std::fs::metadata("tests/test-ASCII.txt")?.file_size();

    let all_chunks_str: String = output.trim().split('\n').collect();
    let all_chunks_vec: Vec<&str> = all_chunks_str.split(' ').collect();

    // Assert that size of file == number of chunks
    assert_eq!(file_len, all_chunks_vec.len() as u64);

    // Assert each chunk is 1 byte (2 ASCIIs)
    all_chunks_vec
        .iter()
        .for_each(|&chunk| assert_eq!(2, chunk.len()));
    Ok(())
}

/**
 * Test: -w flag
 * Ensure each line has only 8 bytes instead of 16
 * (Disables offset for simplicity)
 */
#[test]
fn test_width() -> Result<(), Box<dyn Error>> {
    // -o -w 8
    let args = Args {
        file: "tests/test-256B.bin".to_string(),
        width: 8,
        offset: false,
        ..Default::default()
    };

    let output = hexdump(args)?;

    output.trim().split('\n').for_each(|line| {
        let joined: String = line.split(' ').collect();
        assert_eq!(16, joined.len());
    });
    Ok(())
}

/**
 * Test: -s flag
 * Ensure that the first bytes read "world".
 * (Disables offset for simplicity)
 */
#[test]
fn test_start_offset() -> Result<(), Box<dyn Error>> {
    // -o -s 6
    let args = Args {
        file: "tests/test-ASCII.txt".to_string(),
        start: 6,
        offset: false,
        ..Default::default()
    };

    let output = hexdump(args)?;

    // Assert output starts with "world"
    assert!(output.starts_with("776f 726c 642e"));

    Ok(())
}
