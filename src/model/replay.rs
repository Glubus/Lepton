//! Replay data structure.

use crate::model::input::ReplayInput;

/// Complete replay data in LEP format.
///
/// This structure represents a universal replay for VSRG games.
///
/// # Format Binaire
///
/// The LEP format encodes data compactly:
/// - Magic bytes: `"LEP\0"` (4 bytes)
/// - Version: 1 byte
/// - Rate: f64 little-endian (8 bytes)
/// - Hash: LEB128 length + UTF-8 string
/// - Inputs: LEB128 count + packed inputs avec delta LEB128
///
/// # Compression
///
/// The final format is compressed with zstd when the `compression` feature is enabled.
#[derive(Debug, Clone, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct ReplayData {
    /// Version of the LEP format (currently 1).
    pub version: u8,

    /// Replay rate.
    /// - 1.0 = normal speed
    /// - 0.5 = half speed
    /// - 2.0 = double speed
    pub rate: f64,

    /// Optional hash of the chart associated with the replay.
    /// Allows linking the replay to a specific chart.
    pub hash: Option<String>,

    /// List of replay inputs.
    /// Inputs are stored with deltas in microseconds.
    pub inputs: Vec<ReplayInput>,
}

impl ReplayData {
    /// Creates a new replay with default parameters.
    ///
    /// # Arguments
    ///
    /// * `inputs` - List of replay inputs
    #[must_use]
    pub fn new(inputs: Vec<ReplayInput>) -> Self {
        Self {
            version: 1,
            rate: 1.0,
            hash: None,
            inputs,
        }
    }

    /// Creates a replay with all parameters.
    #[must_use]
    pub fn with_params(rate: f64, hash: Option<String>, inputs: Vec<ReplayInput>) -> Self {
        Self {
            version: 1,
            rate,
            hash,
            inputs,
        }
    }
}
