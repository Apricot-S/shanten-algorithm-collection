use common::shanten_tests;
use common::{ShantenCalculator, TileCounts};

struct DecompKobayashi {}

impl ShantenCalculator for DecompKobayashi {
    fn new() -> Self {
        // TODO:
        DecompKobayashi {}
    }

    fn calculate_shanten(&self, _hand: &TileCounts) -> i8 {
        // TODO:
        0
    }
}

shanten_tests!(DecompKobayashi);
