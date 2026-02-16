pub mod decoder;
pub mod encoder;
pub mod parser;
pub mod types;

pub use decoder::OsuDecoder;
pub use encoder::OsuEncoder;

#[cfg(test)]
mod tests;

use crate::codec::traits::Format;

pub struct OsuFormat;

impl Format for OsuFormat {
    const EXTENSIONS: &'static [&'static str] = &["osr"];
}
