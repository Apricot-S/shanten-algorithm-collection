#![feature(test)]

extern crate test;

use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};
use common::{shanten_benches, shanten_tests};

const MAX_SHANTEN: i8 = 8;
const NUM_CHOW_TYPES: usize = 7 * 3;
const NUM_MELD_TYPES: usize = NUM_TILE_TYPE + NUM_CHOW_TYPES;

type Meld = [usize; 3];

const fn make_melds() -> [Meld; NUM_MELD_TYPES] {
    let mut melds = [[0; 3]; NUM_MELD_TYPES];
    let mut tile = 0;
    while tile < NUM_TILE_TYPE {
        melds[tile] = [tile, tile, tile];
        tile += 1;
    }

    let mut chow_id = 0;
    while chow_id < NUM_CHOW_TYPES {
        let suit = chow_id / 7;
        let rank = chow_id % 7;
        let start = suit * 9 + rank;
        melds[NUM_TILE_TYPE + chow_id] = [start, start + 1, start + 2];
        chow_id += 1;
    }
    melds
}

const MELDS: [Meld; NUM_MELD_TYPES] = make_melds();

/// Shanten-only derivative of the pruning DFS in `mjai-manue-go`.
#[derive(Default)]
pub struct PruningDfsMjaiManueGo;

impl PruningDfsMjaiManueGo {
    fn search(
        current: &TileCounts,
        target: &mut TileCounts,
        current_shanten: i8,
        melds_left: u8,
        min_meld_id: usize,
        mut upper_bound: i8,
    ) -> i8 {
        if melds_left == 0 {
            for tile in 0..NUM_TILE_TYPE {
                if target[tile] > 2 {
                    continue;
                }
                let pair_distance = (target[tile] + 2)
                    .saturating_sub(current[tile])
                    .cast_signed();
                upper_bound = upper_bound.min(current_shanten + pair_distance);
            }
            return upper_bound;
        }

        for (meld_id, meld) in MELDS
            .iter()
            .enumerate()
            .take(NUM_TILE_TYPE)
            .skip(min_meld_id)
        {
            let tile = meld[0];
            if target[tile] >= 2 {
                continue;
            }
            let distance = if current[tile] > target[tile] {
                (target[tile] + 3)
                    .saturating_sub(current[tile])
                    .cast_signed()
            } else {
                3
            };
            let new_shanten = current_shanten + distance;
            if distance < 3 && new_shanten <= upper_bound {
                target[tile] += 3;
                upper_bound = Self::search(
                    current,
                    target,
                    new_shanten,
                    melds_left - 1,
                    meld_id + 1,
                    upper_bound,
                );
                target[tile] -= 3;
            }
        }

        let first_chow = min_meld_id.saturating_sub(NUM_TILE_TYPE);
        for chow_id in first_chow..NUM_CHOW_TYPES {
            let meld_id = NUM_TILE_TYPE + chow_id;
            let [first, second, third] = MELDS[meld_id];
            if target[first] >= 4 || target[second] >= 4 || target[third] >= 4 {
                continue;
            }

            let distance = i8::from(current[first] <= target[first])
                + i8::from(current[second] <= target[second])
                + i8::from(current[third] <= target[third]);
            let new_shanten = current_shanten + distance;
            if distance < 3 && new_shanten <= upper_bound {
                target[first] += 1;
                target[second] += 1;
                target[third] += 1;
                upper_bound = Self::search(
                    current,
                    target,
                    new_shanten,
                    melds_left - 1,
                    meld_id,
                    upper_bound,
                );
                target[first] -= 1;
                target[second] -= 1;
                target[third] -= 1;
            }
        }

        upper_bound
    }
}

impl ShantenCalculator for PruningDfsMjaiManueGo {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let melds_left = (hand.iter().sum::<TileCount>() / 3).min(4);
        Self::search(
            hand,
            &mut [0; NUM_TILE_TYPE],
            -1,
            melds_left,
            0,
            MAX_SHANTEN,
        )
    }
}

shanten_tests!(PruningDfsMjaiManueGo);
shanten_benches!(PruningDfsMjaiManueGo);

#[cfg(test)]
mod differential_tests {
    use std::fs;

    use common::ShantenCalculator;
    use decomp_fixed_pruned::DecompFixedPruned;
    use pruning_dfs_ymatsux::PruningDfsYmatsux;

    use super::*;

    const RESOURCE_FILES: [&str; 4] = [
        "../../resources/hands_normal_10000.txt",
        "../../resources/hands_half_flush_10000.txt",
        "../../resources/hands_full_flush_10000.txt",
        "../../resources/hands_thirteen_orphans_10000.txt",
    ];

    fn parse_hand(line: &str) -> TileCounts {
        let mut hand = [0; NUM_TILE_TYPE];
        for token in line.split_whitespace() {
            let tile: usize = token.parse().expect("resource tile must be an integer");
            hand[tile] += 1;
        }
        hand
    }

    #[test]
    #[ignore = "expensive 40,000-hand differential validation"]
    fn matches_exact_implementations_on_resources() {
        let derived = PruningDfsMjaiManueGo;
        let ymatsux = PruningDfsYmatsux::default();
        let fixed = DecompFixedPruned;

        for filename in RESOURCE_FILES {
            let input = fs::read_to_string(filename).expect("resource file must be readable");
            for (line_number, line) in input.lines().enumerate() {
                let hand = parse_hand(line);
                let actual = derived.calculate_shanten(&hand);
                assert_eq!(
                    actual,
                    ymatsux.calculate_shanten(&hand),
                    "ymatsux mismatch in {filename}:{}",
                    line_number + 1
                );
                assert_eq!(
                    actual,
                    fixed.calculate_shanten(&hand),
                    "fixed decomposition mismatch in {filename}:{}",
                    line_number + 1
                );
            }
        }
    }
}
