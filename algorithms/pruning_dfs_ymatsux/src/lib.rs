use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};

const MAX_SHANTEN: i8 = 8;
const NUM_MELD_TYPE: usize = NUM_TILE_TYPE + 7 * 3;

type Meld = [usize; 3];

fn add_meld(target: &mut TileCounts, meld: &Meld) {
    for &tile in meld {
        target[tile] += 1;
    }
}

fn remove_meld(target: &mut TileCounts, meld: &Meld) {
    for &tile in meld {
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

/// Pruning DFS devised by Yoshitake Matsumoto (ymatsux).
pub struct PruningDfsYmatsux {
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
            for i in 0..NUM_TILE_TYPE {
                target[i] += 2;
                if is_valid_hand(target) {
                    upper_bound = upper_bound.min(calculate_shanten_number(hand, target));
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
                    upper_bound = upper_bound.min(self.calculate_shanten_impl(
                        hand,
                        target,
                        num_left_meld - 1,
                        i,
                        upper_bound,
                    ));
                }
            }
            remove_meld(target, &self.melds[i]);
        }
        upper_bound
    }
}

impl Default for PruningDfsYmatsux {
    fn default() -> Self {
        let mut melds = [[0; 3]; NUM_MELD_TYPE];
        for (i, meld) in melds.iter_mut().take(NUM_TILE_TYPE).enumerate() {
            *meld = [i, i, i];
        }
        let sequence_starts = (0..27).filter(|tile| tile % 9 < 7);
        for (meld, tile) in melds.iter_mut().skip(NUM_TILE_TYPE).zip(sequence_starts) {
            *meld = [tile, tile + 1, tile + 2];
        }
        Self { melds }
    }
}

impl ShantenCalculator for PruningDfsYmatsux {
    fn new() -> Self {
        Self::default()
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let mut target = [0; NUM_TILE_TYPE];
        let num_left_meld = hand.iter().sum::<TileCount>() / 3;
        self.calculate_shanten_impl(hand, &mut target, num_left_meld, 0, MAX_SHANTEN)
    }
}

common::shanten_tests!(PruningDfsYmatsux);
