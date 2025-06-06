use crate::types::TileCounts;

pub trait ShantenCalculator {
    fn new() -> Self;
    fn calculate_shanten(&self, hand: &TileCounts) -> i8;
}
