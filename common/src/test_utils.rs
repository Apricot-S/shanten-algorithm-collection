use crate::constants::NUM_TILE_TYPE;
use crate::types::TileCounts;

/// Extension trait for [`TileCounts`] to support conversion from Tenhou-style hand strings.
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
    /// The [`TileCounts`] array representing the hand's tile counts.
    fn from_code(hand: &str) -> TileCounts;
}

impl TileCountsExt for TileCounts {
    fn from_code(hand: &str) -> TileCounts {
        let mut suit_offset = None;
        let mut counts: TileCounts = [0; NUM_TILE_TYPE];

        for byte in hand.bytes().rev() {
            match byte {
                b'm' => suit_offset = Some(0),
                b'p' => suit_offset = Some(9),
                b's' => suit_offset = Some(18),
                b'z' => suit_offset = Some(27),
                b'0'..=b'9' => {
                    let number = byte - b'0';

                    assert!(
                        (1..=9).contains(&number),
                        "tile number must be between 1 and 9, got {number}"
                    );
                    let base = suit_offset.expect("no type specified before the tile number");
                    let tile_index = base + usize::from(number - 1);
                    counts[tile_index] += 1;
                }
                _ => {}
            }
        }

        counts
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
    #[should_panic(expected = "tile number must be between 1 and 9")]
    fn test_from_code_offset_out_of_range_number() {
        // 0m does not exist
        TileCounts::from_code("0m");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_from_code_offset_out_of_range_z() {
        // 8z does not exist
        TileCounts::from_code("8z");
    }
}
