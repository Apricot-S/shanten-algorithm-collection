use common::shanten_tests;
use common::{ShantenCalculator, TileCounts};

struct Dummy {}

impl ShantenCalculator for Dummy {
    fn new() -> Self {
        Dummy {} // No special initialization for dummy implementation
    }

    /// Dummy implementation for calculating shanten number
    fn calculate_shanten(&self, _hand: &TileCounts) -> i8 {
        0 // Dummy implementation
    }
}

// Generate test cases using macro
shanten_tests!(Dummy);
