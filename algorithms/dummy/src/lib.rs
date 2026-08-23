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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_returns_zero() {
        assert_eq!(Dummy.calculate_shanten(&[0; 34]), 0);
    }
}

// Generate benchmarks using macro
shanten_benches!(Dummy);
