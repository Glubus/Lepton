//! LEB128 encoding and decoding utilities.
//!
//! LEB128 (Little Endian Base 128) is a variable-length encoding for unsigned integers.
//! Small values (0-127) use 1 byte, larger values use more bytes as needed.
//!
//! This is used extensively in the LEP format to compress:
//! - Hash string length
//! - Input count
//! - Delta microseconds between inputs
use std::io::Cursor;

use crate::error::{LeptonError, LeptonResult};

/// Writes a u64 value in LEB128 format to a buffer.
///
/// # Arguments
///
/// * `value` - The unsigned 64-bit integer to encode
/// * `buffer` - The buffer to write bytes into
///
/// # Examples
///
/// ```ignore
/// let mut buf = Vec::new();
/// write_leb128(127, &mut buf);
/// assert_eq!(buf, vec![127]); // Values 0-127 use 1 byte
///
/// let mut buf = Vec::new();
/// write_leb128(300, &mut buf);
/// assert_eq!(buf.len(), 2); // Larger values use multiple bytes
/// ```
pub fn write_leb128(value: u64, buffer: &mut Vec<u8>) {
    leb128::write::unsigned(buffer, value).expect("Writing to Vec should never fail");
}

/// Reads a u64 value in LEB128 format from a cursor.
///
/// # Arguments
///
/// * `cursor` - The cursor to read from
///
/// # Returns
///
/// The decoded u64 value
///
/// # Errors
///
/// Returns `LeptonError::Leb128Read` if the data is invalid or truncated.
pub fn read_leb128(cursor: &mut Cursor<&[u8]>) -> LeptonResult<u64> {
    leb128::read::unsigned(cursor).map_err(LeptonError::Leb128Read)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leb128_small_values() {
        // Values 0-127 should encode to 1 byte
        for val in 0..=127 {
            let mut buf = Vec::new();
            write_leb128(val, &mut buf);
            assert_eq!(buf.len(), 1);

            let mut cursor = Cursor::new(buf.as_slice());
            let decoded = read_leb128(&mut cursor).unwrap();
            assert_eq!(decoded, val);
        }
    }

    #[test]
    fn test_leb128_large_values() {
        // Values > 127 use multiple bytes
        let test_cases = vec![
            (128, 2),       // 2 bytes
            (255, 2),       // 2 bytes
            (16383, 2),     // 2 bytes
            (16384, 3),     // 3 bytes
            (100_000, 3),   // Typical delta microsecond value
            (1_000_000, 3), // 1 second in microseconds
        ];

        for (val, expected_bytes) in test_cases {
            let mut buf = Vec::new();
            write_leb128(val, &mut buf);
            assert_eq!(
                buf.len(),
                expected_bytes,
                "Value {} should encode to {} bytes",
                val,
                expected_bytes
            );

            let mut cursor = Cursor::new(buf.as_slice());
            let decoded = read_leb128(&mut cursor).unwrap();
            assert_eq!(decoded, val);
        }
    }

    #[test]
    fn test_leb128_roundtrip() {
        // Test a variety of values for roundtrip encoding/decoding
        let test_values = vec![
            0,
            1,
            127,
            128,
            255,
            256,
            1000,
            10_000,
            100_000,
            1_000_000,
            u64::MAX,
        ];

        for val in test_values {
            let mut buf = Vec::new();
            write_leb128(val, &mut buf);

            let mut cursor = Cursor::new(buf.as_slice());
            let decoded = read_leb128(&mut cursor).unwrap();
            assert_eq!(decoded, val, "Roundtrip failed for value {}", val);
        }
    }

    #[test]
    fn test_leb128_empty_buffer() {
        let buf = vec![];
        let mut cursor = Cursor::new(buf.as_slice());
        let result = read_leb128(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn test_leb128_truncated() {
        // Create an incomplete LEB128 sequence (continuation bit set but no next byte)
        let buf = vec![0x80]; // Continuation bit set, but no following byte
        let mut cursor = Cursor::new(buf.as_slice());
        let result = read_leb128(&mut cursor);
        assert!(result.is_err());
    }
}
