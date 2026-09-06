use common::{NUM_TILE_TYPES, ShantenCalculator, TileCount, TileCounts};

const MAX_SHANTEN: i8 = 8;

fn calculate_shanten_from_counts(mut melds: i8, mut meld_candidates: i8, has_pair: bool) -> i8 {
    // Adjust for excess melds
    if melds > 4 {
        meld_candidates += melds - 4;
        melds = 4;
    }
    // Adjust for excess meld candidates
    if melds + meld_candidates > 4 {
        meld_candidates = 4 - melds;
    }
    // Count the pair as a meld candidate if it exists
    if has_pair {
        meld_candidates += 1;
    }
    MAX_SHANTEN - melds * 2 - meld_candidates
}

struct BlockCounts {
    melds: i8,
    meld_candidates: i8,
}

struct BlockCountPatterns {
    /// Pattern with the minimum number of isolated tiles
    a: BlockCounts,
    /// Pattern with the maximum number of melds
    b: BlockCounts,
}

impl BlockCounts {
    fn is_a_better_than(&self, other: &BlockCounts) -> bool {
        self.melds * 2 + self.meld_candidates > other.melds * 2 + other.meld_candidates
    }

    fn is_b_better_than(&self, other: &BlockCounts) -> bool {
        self.melds * 10 + self.meld_candidates > other.melds * 10 + other.meld_candidates
    }
}

fn count_meld_candidates(single_color_hand: &mut [TileCount], n: usize) -> BlockCountPatterns {
    if n >= 9 {
        return BlockCountPatterns {
            a: BlockCounts {
                melds: 0,
                meld_candidates: 0,
            },
            b: BlockCounts {
                melds: 0,
                meld_candidates: 0,
            },
        };
    }

    let mut max = count_meld_candidates(single_color_hand, n + 1);

    // edge joint or open joint
    if n < 8 && single_color_hand[n] > 0 && single_color_hand[n + 1] > 0 {
        single_color_hand[n] -= 1;
        single_color_hand[n + 1] -= 1;
        let mut r = count_meld_candidates(single_color_hand, n);
        single_color_hand[n + 1] += 1;
        single_color_hand[n] += 1;

        r.a.meld_candidates += 1;
        r.b.meld_candidates += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a;
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b;
        }
    }

    // middle joint
    if n < 7 && single_color_hand[n] > 0 && single_color_hand[n + 2] > 0 {
        single_color_hand[n] -= 1;
        single_color_hand[n + 2] -= 1;
        let mut r = count_meld_candidates(single_color_hand, n);
        single_color_hand[n + 2] += 1;
        single_color_hand[n] += 1;

        r.a.meld_candidates += 1;
        r.b.meld_candidates += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a;
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b;
        }
    }

    // pair (triplet candidate)
    if single_color_hand[n] == 2 {
        single_color_hand[n] -= 2;
        let mut r = count_meld_candidates(single_color_hand, n);
        single_color_hand[n] += 2;

        r.a.meld_candidates += 1;
        r.b.meld_candidates += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a;
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b;
        }
    }

    max
}

fn count_suit_blocks(single_color_hand: &mut [TileCount], n: usize) -> BlockCountPatterns {
    if n >= 9 {
        return count_meld_candidates(single_color_hand, 0);
    }

    let mut max = count_suit_blocks(single_color_hand, n + 1);

    // sequence
    if n < 7
        && single_color_hand[n] > 0
        && single_color_hand[n + 1] > 0
        && single_color_hand[n + 2] > 0
    {
        single_color_hand[n] -= 1;
        single_color_hand[n + 1] -= 1;
        single_color_hand[n + 2] -= 1;
        let mut r = count_suit_blocks(single_color_hand, n);
        single_color_hand[n + 2] += 1;
        single_color_hand[n + 1] += 1;
        single_color_hand[n] += 1;

        r.a.melds += 1;
        r.b.melds += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a;
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b;
        }
    }

    // triplet
    if single_color_hand[n] >= 3 {
        single_color_hand[n] -= 3;
        let mut r = count_suit_blocks(single_color_hand, n);
        single_color_hand[n] += 3;

        r.a.melds += 1;
        r.b.melds += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a;
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b;
        }
    }

    max
}

fn count_honor_blocks(honor_hand: &[TileCount]) -> BlockCounts {
    let mut melds = 0;
    let mut meld_candidates = 0;

    for c in honor_hand {
        match c {
            3.. => melds += 1,
            2 => meld_candidates += 1,
            0 | 1 => (),
        }
    }

    BlockCounts {
        melds,
        meld_candidates,
    }
}

fn calculate_shanten_impl(hand: &mut TileCounts, has_pair: bool, called_melds: i8) -> i8 {
    let manzu_counts = count_suit_blocks(&mut hand[0..9], 0);
    let pinzu_counts = count_suit_blocks(&mut hand[9..18], 0);
    let souzu_counts = count_suit_blocks(&mut hand[18..27], 0);
    let z = count_honor_blocks(&hand[27..34]);

    let mut min = MAX_SHANTEN;

    for m in [&manzu_counts.a, &manzu_counts.b] {
        for p in [&pinzu_counts.a, &pinzu_counts.b] {
            for s in [&souzu_counts.a, &souzu_counts.b] {
                let melds = called_melds + m.melds + p.melds + s.melds + z.melds;
                let meld_candidates =
                    m.meld_candidates + p.meld_candidates + s.meld_candidates + z.meld_candidates;
                let shanten = calculate_shanten_from_counts(melds, meld_candidates, has_pair);
                min = min.min(shanten);
            }
        }
    }

    min
}

/// Ara's block-decomposition algorithm.
pub struct DecompAra;

impl ShantenCalculator for DecompAra {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_melds = (hand.iter().sum::<TileCount>() / 3).cast_signed();
        let called_melds = 4 - required_melds;
        let mut hand_clone = *hand;

        // Calculate the shanten number without a pair
        let mut min = calculate_shanten_impl(&mut hand_clone, false, called_melds);

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPES {
            if hand_clone[i] >= 2 {
                hand_clone[i] -= 2;
                let temp = calculate_shanten_impl(&mut hand_clone, true, called_melds);
                hand_clone[i] += 2;
                min = min.min(temp);
            }
        }

        min
    }
}

common::shanten_tests!(
    DecompAra,
    profile = legacy_decomposition,
    reason = "the original algorithm does not correct for insufficient isolated tiles"
);
