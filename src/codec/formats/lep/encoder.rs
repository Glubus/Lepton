//! LEP format encoder.
//!
//! Encodes `ReplayData` into the compact LEP binary format with:
//! - Magic bytes: `"LEP\0"`
//! - Version: 1 byte
//! - Rate: f64 (8 bytes, little-endian)
//! - Hash: LEB128 length + UTF-8 string
//! - Inputs: LEB128 count + (LEB128 delta + packed byte) per input
//! - zstd compression (mandatory)

use crate::codec::traits::Encoder;
use crate::error::LeptonResult;
use crate::model::ReplayData;

use super::leb128_utils::write_leb128;

/// LEP format encoder.
pub struct LepEncoder;

/// Magic bytes for LEP format: "LEP\0"
const MAGIC: &[u8; 4] = b"LEP\0";

/// Current LEP format version
const VERSION: u8 = 1;

/// Writes the LEP header (magic, version, rate) to buffer.
fn write_header(replay: &ReplayData, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(MAGIC);
    buffer.push(VERSION);
    buffer.extend_from_slice(&replay.rate.to_le_bytes());
}

/// Writes the hash (LEB128 length + UTF-8 bytes) to buffer.
fn write_hash(hash: &Option<String>, buffer: &mut Vec<u8>) {
    if let Some(hash) = hash {
        let hash_bytes = hash.as_bytes();
        write_leb128(hash_bytes.len() as u64, buffer);
        buffer.extend_from_slice(hash_bytes);
    } else {
        write_leb128(0, buffer);
    }
}

/// Writes all inputs (count + delta/packed per input) to buffer.
fn write_inputs(replay: &ReplayData, buffer: &mut Vec<u8>) {
    write_leb128(replay.inputs.len() as u64, buffer);
    for input in &replay.inputs {
        write_leb128(input.delta_us, buffer);
        buffer.push(input.packed);
    }
}

impl Encoder for LepEncoder {
    fn encode(replay: &ReplayData) -> LeptonResult<Vec<u8>> {
        let mut buffer = Vec::new();

        write_header(replay, &mut buffer);
        write_hash(&replay.hash, &mut buffer);
        write_inputs(replay, &mut buffer);

        let compressed = zstd::encode_all(buffer.as_slice(), 3)?;
        Ok(compressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ReplayInput;

    // Note: These tests verify that encoding works, but cannot check raw binary format
    // since compression is mandatory. Use roundtrip decoder tests to verify correctness.

    #[test]
    fn test_encode_empty_replay() {
        let replay = ReplayData::new(vec![]);
        let data = LepEncoder::encode(&replay).unwrap();

        // Data should be compressed, so we can't check magic bytes directly
        // Just verify it encodes without error
        assert!(!data.is_empty());
    }

    #[test]
    fn test_encode_with_hash() {
        let replay = ReplayData::with_params(1.0, Some("abc123".to_string()), vec![]);
        let data = LepEncoder::encode(&replay).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_encode_simple_inputs() {
        let inputs = vec![
            ReplayInput::new(1000, 0, true, false), // Delta 1000us, column 0, press
            ReplayInput::new(500, 2, false, false), // Delta 500us, column 2, release
        ];
        let replay = ReplayData::new(inputs);
        let data = LepEncoder::encode(&replay).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_encode_with_rate() {
        let replay = ReplayData::with_params(1.5, None, vec![]);
        let data = LepEncoder::encode(&replay).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_encode_auto_flag() {
        let inputs = vec![
            ReplayInput::new(100, 5, true, true), // Auto-generated input
        ];
        let replay = ReplayData::new(inputs);
        let data = LepEncoder::encode(&replay).unwrap();
        assert!(!data.is_empty());
    }
}
