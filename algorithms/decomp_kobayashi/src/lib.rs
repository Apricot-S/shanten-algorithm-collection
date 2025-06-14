#![feature(test)]

extern crate test;

use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};
use common::{shanten_benches, shanten_tests};

const MAX_SHANTEN: i8 = 13;

fn formula(mut num_meld: i8, mut num_meld_cand: i8, mut num_isolated: i8, has_pair: bool) -> i8 {
    let num_blocks = if has_pair { 4 } else { 5 };
    // Adjust for excess melds
    if num_meld > 4 {
        num_meld_cand += num_meld - 4;
        num_meld = 4;
    }
    // Adjust for excess meld candidates
    if num_meld + num_meld_cand > 4 {
        num_isolated += num_meld + num_meld_cand - 4;
        num_meld_cand = 4 - num_meld;
    }
    // Adjust for excess isolated tiles
    if num_meld + num_meld_cand + num_isolated > num_blocks {
        num_isolated = num_blocks - num_meld - num_meld_cand;
    }
    // Count the pair as a meld candidate if it exists
    if has_pair {
        num_meld_cand += 1;
    }
    MAX_SHANTEN - num_meld * 3 - num_meld_cand * 2 - num_isolated
}

struct NumBlocks {
    num_meld: i8,
    num_meld_cand: i8,
    num_isolated: i8,
}

struct NumBlocksPattern {
    /// Pattern with the minimum number of isolated tiles
    a: NumBlocks,
    /// Pattern with the maximum number of melds
    b: NumBlocks,
}

impl NumBlocks {
    fn is_a_better_than(&self, other: &NumBlocks) -> bool {
        self.num_isolated < other.num_isolated
            || self.num_isolated == other.num_isolated && self.num_meld_cand < other.num_meld_cand
    }

    fn is_b_better_than(&self, other: &NumBlocks) -> bool {
        self.num_meld > other.num_meld
            || self.num_meld == other.num_meld && self.num_meld_cand > other.num_meld_cand
    }
}

fn count_num_meld_cand(single_color_hand: &[TileCount]) -> NumBlocksPattern {
    let mut num_tiles = 0;
    let mut num_meld_cand = 0;
    let mut num_isolated = 0;

    for i in 0..9 {
        num_tiles += single_color_hand[i] as i8;
        if i < 7 && single_color_hand[i + 1] == 0 && single_color_hand[i + 2] == 0 {
            num_meld_cand += num_tiles / 2;
            num_isolated += num_tiles % 2;
            num_tiles = 0;
        }
    }

    num_meld_cand += num_tiles / 2;
    num_isolated += num_tiles % 2;

    NumBlocksPattern {
        a: NumBlocks {
            num_meld: 0,
            num_meld_cand,
            num_isolated,
        },
        b: NumBlocks {
            num_meld: 0,
            num_meld_cand,
            num_isolated,
        },
    }
}

fn count_suit_num_blocks(single_color_hand: &mut [TileCount], n: usize) -> NumBlocksPattern {
    if n >= 9 {
        return count_num_meld_cand(single_color_hand);
    }

    let mut max = count_suit_num_blocks(single_color_hand, n + 1);

    // sequence
    if n < 7
        && single_color_hand[n] > 0
        && single_color_hand[n + 1] > 0
        && single_color_hand[n + 2] > 0
    {
        single_color_hand[n] -= 1;
        single_color_hand[n + 1] -= 1;
        single_color_hand[n + 2] -= 1;
        let mut r = count_suit_num_blocks(single_color_hand, n);
        single_color_hand[n + 2] += 1;
        single_color_hand[n + 1] += 1;
        single_color_hand[n] += 1;

        r.a.num_meld += 1;
        r.b.num_meld += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b
        }
    }

    // triplet
    if single_color_hand[n] >= 3 {
        single_color_hand[n] -= 3;
        let mut r = count_suit_num_blocks(single_color_hand, n);
        single_color_hand[n] += 3;

        r.a.num_meld += 1;
        r.b.num_meld += 1;
        if r.a.is_a_better_than(&max.a) {
            max.a = r.a
        }
        if r.b.is_b_better_than(&max.b) {
            max.b = r.b
        }
    }

    max
}

fn count_honor_num_blocks(honor_hand: &[TileCount]) -> NumBlocks {
    let mut num_meld = 0;
    let mut num_meld_cand = 0;
    let mut num_isolated = 0;

    for c in honor_hand {
        match c {
            3.. => num_meld += 1,
            2 => num_meld_cand += 1,
            1 => num_isolated += 1,
            0 => (),
        }
    }

    NumBlocks {
        num_meld,
        num_meld_cand,
        num_isolated,
    }
}

fn calculate_shanten_impl(hand: &mut TileCounts, has_pair: bool, num_call: i8) -> i8 {
    let num_blocks_m = count_suit_num_blocks(&mut hand[0..9], 0);
    let num_blocks_p = count_suit_num_blocks(&mut hand[9..18], 0);
    let num_blocks_s = count_suit_num_blocks(&mut hand[18..27], 0);
    let z = count_honor_num_blocks(&hand[27..34]);

    let mut min = MAX_SHANTEN;

    for m in [&num_blocks_m.a, &num_blocks_m.b] {
        for p in [&num_blocks_p.a, &num_blocks_p.b] {
            for s in [&num_blocks_s.a, &num_blocks_s.b] {
                let num_meld = num_call + m.num_meld + p.num_meld + s.num_meld + z.num_meld;
                let num_meld_cand =
                    m.num_meld_cand + p.num_meld_cand + s.num_meld_cand + z.num_meld_cand;
                let num_isolated =
                    m.num_isolated + p.num_isolated + s.num_isolated + z.num_isolated;
                let shanten = formula(num_meld, num_meld_cand, num_isolated, has_pair);
                min = min.min(shanten);
            }
        }
    }

    min
}

struct DecompKobayashi {}

impl ShantenCalculator for DecompKobayashi {
    fn new() -> Self {
        DecompKobayashi {}
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_num_meld = (hand.iter().sum::<TileCount>() / 3) as i8;
        let num_call = 4 - required_num_meld;
        let mut hand_clone = *hand;

        // Calculate the shanten number without a pair
        let mut min = calculate_shanten_impl(&mut hand_clone, false, num_call);

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPE {
            if hand_clone[i] >= 2 {
                hand_clone[i] -= 2;
                let temp = calculate_shanten_impl(&mut hand_clone, true, num_call);
                hand_clone[i] += 2;
                min = min.min(temp);
            }
        }

        min
    }
}

shanten_tests!(DecompKobayashi);
shanten_benches!(DecompKobayashi);
