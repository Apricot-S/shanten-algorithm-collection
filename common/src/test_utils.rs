use crate::types::TileCounts;

/// Extension trait for [TileCounts] to support conversion from Tenhou-style hand strings.
///
/// This trait is intended for internal use in macros and test utilities.
/// Algorithm implementers do not need to use this trait directly.
pub trait TileCountsExt {
    /// Converts a Tenhou-style hand string into an array representing
    /// the counts of 34 types of tiles.
    ///
    /// # Arguments
    ///
    /// * `hand` - A string representing the hand (e.g., "123m456p789s12344z")
    ///
    /// # Returns
    ///
    /// The [TileCounts] struct representing the hand's tile counts.
    fn from_code(hand: &str) -> TileCounts;
}

impl TileCountsExt for TileCounts {
    fn from_code(hand: &str) -> TileCounts {
        const TILE_MAP: [(char, usize); 4] = [('m', 0), ('p', 9), ('s', 18), ('z', 27)];
        let mut current_type: Option<usize> = None;
        let mut result: TileCounts = [0u8; 34];

        for c in hand.chars().rev() {
            if let Some(&(_, idx)) = TILE_MAP.iter().find(|&&(t, _)| t == c) {
                current_type = Some(idx);
            } else if let Some(d) = c.to_digit(10) {
                if !(1..=9).contains(&d) {
                    panic!("tile number must be between 1 and 9, got {}", d);
                }
                let base = current_type.expect("no type specified before the tile number");
                let offset = d as usize - 1;
                result[base + offset] += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code_normal() {
        let counts = TileCounts::from_code("123m456p789s12344z");
        let expected_counts: TileCounts = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            1, 1, 1, 2, 0, 0, 0, // z
        ];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    fn test_from_code_multiple_types() {
        let counts = TileCounts::from_code("11m22p33s44z11m2p7s");
        let expected_counts: TileCounts = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 3, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 2, 0, 0, 0, 1, 0, 0, // s
            0, 0, 0, 2, 0, 0, 0, // z
        ];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    fn test_from_code_empty() {
        let counts = TileCounts::from_code("");
        let expected_counts: TileCounts = [0u8; 34];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    #[should_panic(expected = "no type specified before the tile number")]
    fn test_from_code_no_type() {
        TileCounts::from_code("123456");
    }

    #[test]
    #[should_panic]
    fn test_from_code_offset_out_of_range_number() {
        // 0m does not exist
        TileCounts::from_code("0m");
    }

    #[test]
    #[should_panic]
    fn test_from_code_offset_out_of_range_z() {
        // 8z does not exist
        TileCounts::from_code("8z");
    }
}
