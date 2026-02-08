//! LEP format decoder.
//!
//! Decodes LEP binary format back into `ReplayData`.
//! Automatically handles zstd decompression (mandatory).

use std::io::Cursor;

use crate::codec::traits::Decoder;
use crate::error::{LeptonError, LeptonResult};
use crate::model::{ReplayData, ReplayInput};

use super::leb128_utils::read_leb128;

/// LEP format decoder.
pub struct LepDecoder;

/// Magic bytes for LEP format: "LEP\0"
const MAGIC: &[u8; 4] = b"LEP\0";

/// Reads and validates magic bytes from data.
fn read_magic(data: &[u8], cursor: &mut Cursor<&[u8]>) -> LeptonResult<()> {
    let mut magic_buf = [0u8; 4];
    if cursor.position() + 4 > data.len() as u64 {
        return Err(LeptonError::InvalidMagic);
    }
    magic_buf.copy_from_slice(&data[0..4]);
    cursor.set_position(4);

    if &magic_buf != MAGIC {
        return Err(LeptonError::InvalidMagic);
    }
    Ok(())
}

/// Reads version byte from data.
fn read_version(data: &[u8], cursor: &mut Cursor<&[u8]>) -> LeptonResult<u8> {
    if cursor.position() + 1 > data.len() as u64 {
        return Err(LeptonError::InvalidMagic);
    }
    let version = data[cursor.position() as usize];
    cursor.set_position(cursor.position() + 1);
    Ok(version)
}

/// Reads rate (f64) from data.
fn read_rate(data: &[u8], cursor: &mut Cursor<&[u8]>) -> LeptonResult<f64> {
    if cursor.position() + 8 > data.len() as u64 {
        return Err(LeptonError::InvalidMagic);
    }
    let rate_bytes: [u8; 8] = data[cursor.position() as usize..cursor.position() as usize + 8]
        .try_into()
        .map_err(|_| LeptonError::InvalidMagic)?;
    let rate = f64::from_le_bytes(rate_bytes);
    cursor.set_position(cursor.position() + 8);
    Ok(rate)
}

/// Reads optional hash from data.
fn read_hash(data: &[u8], cursor: &mut Cursor<&[u8]>) -> LeptonResult<Option<String>> {
    let hash_len = read_leb128(cursor)? as usize;
    if hash_len == 0 {
        return Ok(None);
    }

    if cursor.position() + hash_len as u64 > data.len() as u64 {
        return Err(LeptonError::InvalidMagic);
    }
    let hash_bytes = &data[cursor.position() as usize..cursor.position() as usize + hash_len];
    cursor.set_position(cursor.position() + hash_len as u64);
    Ok(Some(String::from_utf8(hash_bytes.to_vec())?))
}

/// Reads all inputs from data.
fn read_inputs(data: &[u8], cursor: &mut Cursor<&[u8]>) -> LeptonResult<Vec<ReplayInput>> {
    let input_count = read_leb128(cursor)? as usize;
    let mut inputs = Vec::with_capacity(input_count);

    for _ in 0..input_count {
        let delta_us = read_leb128(cursor)?;

        if cursor.position() + 1 > data.len() as u64 {
            return Err(LeptonError::InvalidMagic);
        }
        let packed = data[cursor.position() as usize];
        cursor.set_position(cursor.position() + 1);

        inputs.push(ReplayInput { delta_us, packed });
    }

    Ok(inputs)
}

impl Decoder for LepDecoder {
    fn decode(data: &[u8]) -> LeptonResult<ReplayData> {
        let raw_data = zstd::decode_all(data)?;
        let mut cursor = Cursor::new(raw_data.as_slice());

        read_magic(&raw_data, &mut cursor)?;
        let version = read_version(&raw_data, &mut cursor)?;
        let rate = read_rate(&raw_data, &mut cursor)?;
        let hash = read_hash(&raw_data, &mut cursor)?;
        let inputs = read_inputs(&raw_data, &mut cursor)?;

        Ok(ReplayData {
            version,
            rate,
            hash,
            inputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::formats::lep::encoder::LepEncoder;
    use crate::codec::traits::Encoder;

    // All tests use roundtrip encoding/decoding since compression is mandatory

    #[test]
    fn test_decode_empty_replay() {
        let replay = ReplayData::new(vec![]);
        let encoded = LepEncoder::encode(&replay).unwrap();
        let decoded = LepDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.rate, 1.0);
        assert_eq!(decoded.hash, None);
        assert_eq!(decoded.inputs.len(), 0);
    }

    #[test]
    fn test_roundtrip_with_hash() {
        let replay = ReplayData::with_params(1.5, Some("test_hash_123".to_string()), vec![]);
        let encoded = LepEncoder::encode(&replay).unwrap();
        let decoded = LepDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.rate, 1.5);
        assert_eq!(decoded.hash, Some("test_hash_123".to_string()));
    }

    #[test]
    fn test_roundtrip_with_inputs() {
        let inputs = vec![
            ReplayInput::new(1000, 0, true, false),
            ReplayInput::new(500, 2, false, false),
            ReplayInput::new(2000, 5, true, true), // With auto flag
        ];
        let replay = ReplayData::new(inputs.clone());
        let encoded = LepEncoder::encode(&replay).unwrap();
        let decoded = LepDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.inputs.len(), 3);
        assert_eq!(decoded.inputs[0].delta_us, 1000);
        assert_eq!(decoded.inputs[1].delta_us, 500);
        assert_eq!(decoded.inputs[2].delta_us, 2000);

        // Verify packed data
        assert_eq!(decoded.inputs[0].column(), 0);
        assert_eq!(decoded.inputs[0].is_press(), true);
        assert_eq!(decoded.inputs[0].is_auto(), false);

        assert_eq!(decoded.inputs[2].is_auto(), true);
    }

    #[test]
    fn test_invalid_magic() {
        let bad_data = b"BAD\0data";
        let result = LepDecoder::decode(bad_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_truncated_data() {
        // Only magic bytes, missing everything else
        let bad_data = b"LEP\0";
        let result = LepDecoder::decode(bad_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip_complete() {
        // Full realistic replay
        let inputs = vec![
            ReplayInput::new(0, 0, true, false),
            ReplayInput::new(100_000, 0, false, false), // 100ms later, release
            ReplayInput::new(50_000, 1, true, false),   // 50ms later, press col 1
            ReplayInput::new(150_000, 1, false, false), // 150ms later, release col 1
        ];
        let replay = ReplayData::with_params(1.0, Some("chart_hash_abc".to_string()), inputs);

        let encoded = LepEncoder::encode(&replay).unwrap();
        let decoded = LepDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded, replay);
    }
}
