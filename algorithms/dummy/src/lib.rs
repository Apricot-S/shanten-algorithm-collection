#![feature(test)]

extern crate test;

use common::shanten_benches;
use common::{ShantenCalculator, TileCounts};

/// Calculator used to demonstrate the benchmark harness.
#[derive(Default)]
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

// Generate benchmarks using macro
shanten_benches!(Dummy);
