use common::{ShantenCalculator, TileCounts};

/// Calculator used to demonstrate the benchmark harness.
pub struct Dummy;

impl ShantenCalculator for Dummy {
    fn new() -> Self {
        Self
    }

    /// Dummy implementation for calculating shanten number
    fn calculate_shanten(&self, _hand: &TileCounts) -> i8 {
        0 // Dummy implementation
    }
}

// Generate the shared correctness tests for an actual implementation.
// common::shanten_tests!(Dummy);
