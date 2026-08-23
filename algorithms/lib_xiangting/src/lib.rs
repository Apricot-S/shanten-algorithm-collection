#![feature(test)]

extern crate test;

use common::{ShantenCalculator, TileCounts, shanten_benches};
use xiangting::{PlayerCount, calculate_replacement_number};

/// Benchmark adapter for the `xiangting` crate.
#[derive(Default)]
pub struct Xiangting;

impl ShantenCalculator for Xiangting {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let replacement_number = calculate_replacement_number(hand, PlayerCount::Four)
            .expect("benchmark data must contain a valid four-player hand");
        i8::try_from(replacement_number).expect("replacement number must fit in i8") - 1
    }
}

shanten_benches!(Xiangting);
