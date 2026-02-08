//! Encoder and Decoder traits for format conversion.

use std::path::Path;

use crate::error::LeptonResult;
use crate::model::ReplayData;

/// Trait for decoding from external formats to ROX.
pub trait Decoder {
    /// Decode a chart from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid or cannot be parsed.
    fn decode(data: &[u8]) -> LeptonResult<ReplayData>;

    /// Decode a chart from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or contains invalid data.
    fn decode_from_path(path: impl AsRef<Path>) -> LeptonResult<ReplayData> {
        let data = std::fs::read(path)?;
        Self::decode(&data)
    }
}

/// Trait for encoding from ROX to external formats.
pub trait Encoder {
    /// Encode a chart to raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the chart is invalid or encoding fails.
    fn encode(chart: &ReplayData) -> LeptonResult<Vec<u8>>;

    /// Encode a chart to a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails or the file cannot be written.
    fn encode_to_path(chart: &ReplayData, path: impl AsRef<Path>) -> LeptonResult<()> {
        let data = Self::encode(chart)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Encode a chart to a String (for text-based formats like .osu).
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails or the output is not valid UTF-8.
    fn encode_to_string(chart: &ReplayData) -> LeptonResult<String> {
        let data = Self::encode(chart)?;
        Ok(String::from_utf8(data)?)
    }
}

/// Trait for formats that support specific file extensions.
/// Implement this trait to enable auto-detection based on file extension.
pub trait Format {
    /// List of supported file extensions (lowercase, without leading dot).
    /// Example: `["osu"]` or `["sm", "ssc"]`
    const EXTENSIONS: &'static [&'static str];

    /// Check if this format supports the given extension.
    #[must_use]
    fn supports_extension(ext: &str) -> bool {
        let ext_lower = ext.to_lowercase();
        Self::EXTENSIONS.iter().any(|&e| e == ext_lower)
    }
}

/// Convert data from one format to another using ROX as the intermediate format.
///
/// # Example
/// ```ignore
/// use rox::codec::{convert, formats::{OsuDecoder, SmEncoder}};
///
/// let osu_bytes = std::fs::read("chart.osu")?;
/// let sm_bytes = convert::<OsuDecoder, SmEncoder>(&osu_bytes)?;
/// ```
///
/// # Errors
///
/// Returns an error if decoding or encoding fails.
pub fn convert<D: Decoder, E: Encoder>(data: &[u8]) -> LeptonResult<Vec<u8>> {
    let chart = D::decode(data)?;
    E::encode(&chart)
}

/// Convert a file from one format to another using ROX as the intermediate format.
///
/// # Example
/// ```ignore
/// use rox::codec::{convert_file, formats::{OsuDecoder, SmEncoder}};
///
/// convert_file::<OsuDecoder, SmEncoder>("chart.osu", "chart.sm")?;
/// ```
///
/// # Errors
///
/// Returns an error if reading, decoding, encoding, or writing fails.
pub fn convert_file<D: Decoder, E: Encoder>(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> LeptonResult<()> {
    let chart = D::decode_from_path(input)?;
    E::encode_to_path(&chart, output)
}
