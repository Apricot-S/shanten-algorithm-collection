use common::{NUM_TILE_TYPES, ShantenCalculator, TileCount, TileCounts};

const MAX_SHANTEN: i8 = 8;
const NUM_CHOW_TYPES: usize = 7 * 3;
const NUM_MELD_TYPES: usize = NUM_TILE_TYPES + NUM_CHOW_TYPES;

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

/// Shanten-only derivative of the pruning DFS in `mjai-manue-go`.
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
            for tile in 0..NUM_TILE_TYPES {
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
            .take(NUM_TILE_TYPES)
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

            if distance < 3 && new_shanten < upper_bound {
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

        let first_chow = min_meld_id.saturating_sub(NUM_TILE_TYPES);
        for chow_id in first_chow..NUM_CHOW_TYPES {
            let meld_id = NUM_TILE_TYPES + chow_id;
            let [first, second, third] = MELDS[meld_id];
            if target[first] >= 4 || target[second] >= 4 || target[third] >= 4 {
                continue;
            }

            let distance = i8::from(current[first] <= target[first])
                + i8::from(current[second] <= target[second])
                + i8::from(current[third] <= target[third]);
            let new_shanten = current_shanten + distance;

            if distance < 3 && new_shanten < upper_bound {
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
        let mut target = [0; NUM_TILE_TYPES];
        let melds_left = hand.iter().sum::<TileCount>() / 3;
        debug_assert!(melds_left <= 4);
        Self::search(hand, &mut target, -1, melds_left, 0, MAX_SHANTEN)
    }
}

common::shanten_tests!(PruningDfsMjaiManueGo);
