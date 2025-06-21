#![feature(test)]

extern crate test;

use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};
use common::{shanten_benches, shanten_tests};

const MAX_SHANTEN: i8 = 8;
const NUM_MELD_TYPE: usize = NUM_TILE_TYPE + 7 * 3;

type Meld = [usize; 3];

fn add_meld(target: &mut TileCounts, meld: &Meld) {
    meld.iter().for_each(|&t| target[t] += 1);
}

fn remove_meld(target: &mut TileCounts, meld: &Meld) {
    meld.iter().for_each(|&t| target[t] -= 1);
}

fn is_valid_hand(hand: &TileCounts) -> bool {
    hand.iter().all(|&n| n <= 4)
}

fn calculate_shanten_number(hand: &TileCounts, target: &TileCounts) -> i8 {
    target
        .iter()
        .zip(hand)
        .map(|(&t, &h)| t.saturating_sub(h))
        .sum::<u8>() as i8
        - 1
}

struct PruningDfsYmatsux {
    melds: [Meld; NUM_MELD_TYPE],
}

impl PruningDfsYmatsux {
    fn calculate_shanten_impl(
        &self,
        hand: &TileCounts,
        target: &mut TileCounts,
        num_left_meld: u8,
        min_meld_id: usize,
        mut upper_bound: i8,
    ) -> i8 {
        if num_left_meld == 0 {
            // Add a pair
            for i in 0..NUM_TILE_TYPE {
                target[i] += 2;
                if is_valid_hand(target) {
                    let shanten_number = calculate_shanten_number(hand, target);
                    upper_bound = upper_bound.min(shanten_number);
                }
                target[i] -= 2;
            }

            return upper_bound;
        }

        for i in min_meld_id..NUM_MELD_TYPE {
            add_meld(target, &self.melds[i]);
            if is_valid_hand(target) {
                let lower_bound = calculate_shanten_number(hand, target);
                if lower_bound < upper_bound {
                    let min_shanten = self.calculate_shanten_impl(
                        hand,
                        target,
                        num_left_meld - 1,
                        i,
                        upper_bound,
                    );
                    upper_bound = upper_bound.min(min_shanten);
                }
            }
            remove_meld(target, &self.melds[i]);
        }

        upper_bound
    }
}

impl ShantenCalculator for PruningDfsYmatsux {
    fn new() -> Self {
        let mut melds = [[0usize; 3]; NUM_MELD_TYPE];

        // triplets
        for i in 0..NUM_TILE_TYPE {
            melds[i] = [i, i, i];
        }

        // sequences
        let sequence_starts: Vec<usize> = (0..(9 * 3)).filter(|&n| n % 9 < 7).collect();
        for (i, &n) in sequence_starts.iter().enumerate() {
            melds[NUM_TILE_TYPE + i] = [n, n + 1, n + 2];
        }

        PruningDfsYmatsux { melds: melds }
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let mut target = [0u8; 34];
        let num_left_meld = (hand.iter().sum::<TileCount>() / 3) as u8;
        self.calculate_shanten_impl(hand, &mut target, num_left_meld, 0, MAX_SHANTEN)
    }
}

shanten_tests!(PruningDfsYmatsux);
shanten_benches!(PruningDfsYmatsux);
