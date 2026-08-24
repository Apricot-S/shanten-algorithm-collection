use common::{ShantenCalculator, TileCount, TileCounts};
use shanten_dp::{Mode, calc_shanten, make_tile_limits};

/// Benchmark adapter for the `shanten-dp` crate.
pub struct ShantenDp {
    tile_limits: [u8; 35],
}

impl Default for ShantenDp {
    fn default() -> Self {
        Self {
            tile_limits: make_tile_limits(false),
        }
    }
}

impl ShantenCalculator for ShantenDp {
    fn new() -> Self {
        Self::default()
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let num_melds = usize::from(hand.iter().sum::<TileCount>() / 3);
        calc_shanten(
            hand,
            &self.tile_limits,
            num_melds,
            Mode::STANDARD,
            false,
            false,
        )
        .expect("validation is disabled")
        .expect("standard mode always produces a result")
    }
}

common::shanten_tests!(ShantenDp);
