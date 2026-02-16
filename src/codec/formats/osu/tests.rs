use super::*;
use crate::codec::traits::{Decoder, Encoder};
use crate::model::{ReplayData, ReplayInput};

#[test]
fn test_osu_roundtrip() {
    // Create a fake Lepton replay
    // Time must be large enough to not be truncated by ms conversion if we want exact equality.
    // 10ms = 10000us.
    let inputs = vec![
        ReplayInput::new(10000, 0, true, false), // +10ms, col 0 press
        ReplayInput::new(5000, 1, true, false),  // +5ms, col 1 press
        ReplayInput::new(10000, 0, false, false), // +10ms, col 0 release
        ReplayInput::new(5000, 1, false, false), // +5ms, col 1 release
    ];

    let original_data = ReplayData::with_params(1.0, Some("hash".to_string()), inputs.clone());

    // Encode to osu! format
    let encoded_bytes = OsuEncoder::encode(&original_data).expect("Encoding failed");

    // Decode back
    let decoded_data = OsuDecoder::decode(&encoded_bytes).expect("Decoding failed");

    // Verify fields
    assert_eq!(decoded_data.hash, original_data.hash);
    assert_eq!(decoded_data.inputs.len(), original_data.inputs.len());

    for (i, (orig, dec)) in original_data
        .inputs
        .iter()
        .zip(decoded_data.inputs.iter())
        .enumerate()
    {
        assert_eq!(
            orig.delta_us, dec.delta_us,
            "Mismatch at input {} (delta)",
            i
        );
        assert_eq!(
            orig.column(),
            dec.column(),
            "Mismatch at input {} (column)",
            i
        );
        assert_eq!(
            orig.is_press(),
            dec.is_press(),
            "Mismatch at input {} (press)",
            i
        );
    }
}

#[test]
fn test_mania_key_limit() {
    // We can't easily construct a bad ReplayInput (>15), but we can try to verify that
    // the system stays robust.
    // Let's test that 16 keys (0-15) work.
    let inputs = vec![ReplayInput::new(1000, 15, true, false)];
    let original = ReplayData::with_params(1.0, None, inputs);
    let encoded = OsuEncoder::encode(&original).expect("Encoding failed");
    let decoded = OsuDecoder::decode(&encoded).expect("Decoding failed");

    assert_eq!(decoded.inputs[0].column(), 15);
}
