//! Core replay input type.

/// A single user input (press or release).
///
/// LEP packed byte format:
/// ```text
/// Bit:  7   6   5   4   3   2   1   0
///      [R] [R] [A] [P] [C] [C] [C] [C]
///
/// C = Column (0-15, 4 bits)
/// P = Press (1) / Release (0)
/// A = Auto flag (1 = auto-generated)
/// R = Reserved (must be 0)
/// ```
#[derive(Debug, Clone, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct ReplayInput {
    /// Delta in microseconds since the previous input.
    /// For the first input, this is the time since the start of the chart.
    pub delta_us: u64,

    /// Packed data: column (4 bits) | press (1 bit) | auto (1 bit) | reserved (2 bits)
    pub packed: u8,
}

impl ReplayInput {
    /// Creates a new input with delta and packed data.
    ///
    /// # Arguments
    ///
    /// * `delta_us` - Microseconds since the previous input
    /// * `column` - Column index (0-15)
    /// * `is_press` - true if press, false if release
    /// * `is_auto` - true if auto-generated
    ///
    /// # Panics
    ///
    /// Panics if column > 15 (exceeds 4 bits).
    #[must_use]
    pub fn new(delta_us: u64, column: u8, is_press: bool, is_auto: bool) -> Self {
        assert!(column <= 15, "Column must be 0-15 (4 bits)");
        let packed = Self::pack(column, is_press, is_auto);
        Self { delta_us, packed }
    }

    /// Encodes column, press and auto flag into a packed byte.
    ///
    /// # Format
    ///
    /// - Bits 0-3: Column (4 bits)
    /// - Bit 4: Press (1 bit)
    /// - Bit 5: Auto (1 bit)
    /// - Bits 6-7: Reserved (0)
    #[inline]
    #[must_use]
    pub fn pack(column: u8, is_press: bool, is_auto: bool) -> u8 {
        (column & 0x0F) | ((is_press as u8) << 4) | ((is_auto as u8) << 5)
    }

    /// Decodes the packed byte into (column, is_press, is_auto).
    #[inline]
    #[must_use]
    pub fn unpack(&self) -> (u8, bool, bool) {
        let column = self.packed & 0x0F;
        let is_press = (self.packed & 0x10) != 0;
        let is_auto = (self.packed & 0x20) != 0;
        (column, is_press, is_auto)
    }

    /// Returns the column index (0-15).
    #[inline]
    #[must_use]
    pub fn column(&self) -> u8 {
        self.packed & 0x0F
    }

    /// Checks if this is a press (true) or release (false).
    #[inline]
    #[must_use]
    pub fn is_press(&self) -> bool {
        (self.packed & 0x10) != 0
    }

    /// Checks if the input is auto-generated.
    #[inline]
    #[must_use]
    pub fn is_auto(&self) -> bool {
        (self.packed & 0x20) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_roundtrip() {
        let packed = ReplayInput::pack(7, true, false);
        let input = ReplayInput {
            delta_us: 1000,
            packed,
        };
        let (col, press, auto) = input.unpack();
        assert_eq!(col, 7);
        assert_eq!(press, true);
        assert_eq!(auto, false);
    }

    #[test]
    fn test_column_bounds() {
        // 4 bits = 0-15
        for col in 0..=15 {
            let packed = ReplayInput::pack(col, false, false);
            let input = ReplayInput {
                delta_us: 0,
                packed,
            };
            assert_eq!(input.column(), col);
        }
    }

    #[test]
    fn test_all_flags() {
        let input = ReplayInput::new(500, 3, true, true);
        assert_eq!(input.column(), 3);
        assert_eq!(input.is_press(), true);
        assert_eq!(input.is_auto(), true);
        assert_eq!(input.delta_us, 500);
    }

    #[test]
    #[should_panic(expected = "Column must be 0-15")]
    fn test_column_overflow() {
        let _ = ReplayInput::new(0, 16, false, false);
    }
}
