use common::{NUM_TILE_TYPES, ShantenCalculator, TileCount, TileCounts};

const MAX_SHANTEN: i8 = 8;
const NUM_MELD_TYPES: usize = NUM_TILE_TYPES + 7 * 3;

type Meld = [usize; 3];

const fn create_melds() -> [Meld; NUM_MELD_TYPES] {
    let mut melds = [[0; 3]; NUM_MELD_TYPES];
    let mut meld_id = 0;

    while meld_id < NUM_TILE_TYPES {
        melds[meld_id] = [meld_id, meld_id, meld_id];
        meld_id += 1;
    }

    let mut tile = 0;
    while tile < 27 {
        if tile % 9 < 7 {
            melds[meld_id] = [tile, tile + 1, tile + 2];
            meld_id += 1;
        }
        tile += 1;
    }

    melds
}

const MELDS: [Meld; NUM_MELD_TYPES] = create_melds();

fn add_meld(target: &mut TileCounts, meld_id: usize) {
    for &tile in &MELDS[meld_id] {
        target[tile] += 1;
    }
}

fn remove_meld(target: &mut TileCounts, meld_id: usize) {
    for &tile in &MELDS[meld_id] {
        target[tile] -= 1;
    }
}

fn is_valid_hand(hand: &TileCounts) -> bool {
    hand.iter().all(|&count| count <= 4)
}

fn calculate_shanten_number(hand: &TileCounts, target: &TileCounts) -> i8 {
    target
        .iter()
        .zip(hand)
        .map(|(&target_count, &hand_count)| target_count.saturating_sub(hand_count))
        .sum::<u8>()
        .cast_signed()
        - 1
}

/// ymatsux pruning DFS with the latest upper bound applied to sibling branches.
pub struct PruningDfsYmatsuxImproved;

impl PruningDfsYmatsuxImproved {
    fn calculate_shanten_impl(
        hand: &TileCounts,
        target: &mut TileCounts,
        melds_left: u8,
        min_meld_id: usize,
        mut upper_bound: i8,
    ) -> i8 {
        if melds_left == 0 {
            for i in 0..NUM_TILE_TYPES {
                target[i] += 2;
                if is_valid_hand(target) {
                    upper_bound = upper_bound.min(calculate_shanten_number(hand, target));
                }
                target[i] -= 2;
            }
            return upper_bound;
        }

        for i in min_meld_id..NUM_MELD_TYPES {
            add_meld(target, i);
            if is_valid_hand(target) {
                let lower_bound = calculate_shanten_number(hand, target);
                if lower_bound < upper_bound {
                    upper_bound = upper_bound.min(Self::calculate_shanten_impl(
                        hand,
                        target,
                        melds_left - 1,
                        i,
                        upper_bound,
                    ));
                }
            }
            remove_meld(target, i);
        }
        upper_bound
    }
}

impl ShantenCalculator for PruningDfsYmatsuxImproved {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let mut target = [0; NUM_TILE_TYPES];
        let melds_left = hand.iter().sum::<TileCount>() / 3;
        debug_assert!(melds_left <= 4);
        Self::calculate_shanten_impl(hand, &mut target, melds_left, 0, MAX_SHANTEN)
    }
}

common::shanten_tests!(PruningDfsYmatsuxImproved);
