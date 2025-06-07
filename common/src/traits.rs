use crate::types::TileCounts;

/// Trait for implementing shanten number calculation algorithms.
///
/// Please implement this trait to add your own algorithm.
pub trait ShantenCalculator {
    /// Creates a new instance.
    ///
    /// If your algorithm requires loading tables or initialization,
    /// please perform it here.
    fn new() -> Self;

    /// Calculates the shanten number of a hand.
    ///
    /// # Arguments
    ///
    /// * `hand` - Reference to a [TileCounts] struct representing the hand's tile counts
    ///
    /// # Returns
    ///
    /// The shanten number.
    /// Return `0` for tenpai (ready hand), and `-1` for a winning hand.
    fn calculate_shanten(&self, hand: &TileCounts) -> i8;
}
