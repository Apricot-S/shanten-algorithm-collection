#![feature(test)]

extern crate test;

use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};
use common::{shanten_benches, shanten_tests};

const MAX_NUM_BLOCKS: i8 = 4;
const MAX_SHANTEN: i8 = 8;

struct NumBlocks {
    num_meld: i8,
    num_meld_cand: i8,
    num_pair: i8,
}

impl NumBlocks {
    fn get_num_blocks(&self) -> i8 {
        self.num_meld + self.num_meld_cand
    }

    fn formula(&self) -> i8 {
        MAX_SHANTEN - self.num_meld * 2 - self.num_meld_cand - self.num_pair
    }

    fn calculate_lower_bound(&self) -> i8 {
        MAX_NUM_BLOCKS - self.num_meld - self.num_pair
    }
}

fn is_suit(index: usize) -> bool {
    index < 27
}

fn is_honor(index: usize) -> bool {
    !is_suit(index)
}

fn cut_meld(
    hand: &mut TileCounts,
    original: &TileCounts,
    num_blocks: &mut NumBlocks,
    min_shanten: &mut i8,
    pair_index: usize,
    i: usize,
) {
    if i >= NUM_TILE_TYPE {
        let lower_bound = num_blocks.calculate_lower_bound();
        cut_meld_cand(
            hand,
            original,
            num_blocks,
            min_shanten,
            pair_index,
            0,
            lower_bound,
        );
        return;
    }

    // triplet
    if hand[i] >= 3 {
        num_blocks.num_meld += 1;
        hand[i] -= 3;
        cut_meld(hand, original, num_blocks, min_shanten, pair_index, i);
        hand[i] += 3;
        num_blocks.num_meld -= 1;
    }

    // sequence
    if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 1] > 0 && hand[i + 2] > 0 {
        num_blocks.num_meld += 1;
        hand[i] -= 1;
        hand[i + 1] -= 1;
        hand[i + 2] -= 1;
        cut_meld(hand, original, num_blocks, min_shanten, pair_index, i);
        hand[i + 2] += 1;
        hand[i + 1] += 1;
        hand[i] += 1;
        num_blocks.num_meld -= 1;
    }

    cut_meld(hand, original, num_blocks, min_shanten, pair_index, i + 1);
}

fn cut_meld_cand(
    hand: &mut TileCounts,
    original: &TileCounts,
    num_blocks: &mut NumBlocks,
    min_shanten: &mut i8,
    pair_index: usize,
    i: usize,
    lower_bound: i8,
) {
    // pruning
    if *min_shanten <= lower_bound {
        return;
    }

    if i >= NUM_TILE_TYPE {
        if (num_blocks.num_meld == 4 && num_blocks.num_meld_cand == 0 && num_blocks.num_pair == 0)
            || (num_blocks.num_meld == 3
                && num_blocks.num_meld_cand == 1
                && num_blocks.num_pair == 0)
        {
            // 孤立牌不足パターン1: 孤立牌 -> 雀頭
            cut_isolated_tile_for_pair(hand, original, num_blocks, min_shanten);
            return;
        } else if num_blocks.num_meld == 3
            && num_blocks.num_meld_cand == 0
            && num_blocks.num_pair == 1
        {
            // 孤立牌不足パターン2: 孤立牌 -> 面子
            cut_isolated_tile_for_meld(hand, original, num_blocks, min_shanten);
            return;
        } else if num_blocks.num_meld == 3
            && num_blocks.num_meld_cand == 0
            && num_blocks.num_pair == 0
        {
            // 孤立牌不足パターン3: 孤立牌1 -> 雀頭, 孤立牌2 -> 面子
            cut_isolated_tile_for_pair_and_meld(hand, original, num_blocks, min_shanten);
            return;
        } else {
            // 孤立牌十分
            *min_shanten = *min_shanten.min(&mut num_blocks.formula());
            return;
        }
    }

    if num_blocks.get_num_blocks() < MAX_NUM_BLOCKS {
        // pair (triplet candidate)
        if hand[i] == 2 && i != pair_index {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 2;
            cut_meld_cand(
                hand,
                original,
                num_blocks,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i] += 2;
            num_blocks.num_meld_cand -= 1;
        }

        // edge joint or open joint
        if is_suit(i) && i % 9 < 8 && hand[i] > 0 && hand[i + 1] > 0 {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 1;
            hand[i + 1] -= 1;
            cut_meld_cand(
                hand,
                original,
                num_blocks,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i + 1] += 1;
            hand[i] += 1;
            num_blocks.num_meld_cand -= 1;
        }

        // middle joint
        if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 2] > 0 {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 1;
            hand[i + 2] -= 1;
            cut_meld_cand(
                hand,
                original,
                num_blocks,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i + 2] += 1;
            hand[i] += 1;
            num_blocks.num_meld_cand -= 1;
        }
    }

    cut_meld_cand(
        hand,
        original,
        num_blocks,
        min_shanten,
        pair_index,
        i + 1,
        lower_bound,
    );
}

fn cut_isolated_tile_for_pair(
    hand: &TileCounts,
    original: &TileCounts,
    num_blocks: &NumBlocks,
    min_shanten: &mut i8,
) {
    for i in 0..NUM_TILE_TYPE {
        if hand[i] > 0 && original[i] < 3 {
            *min_shanten = *min_shanten.min(&mut num_blocks.formula());
            return;
        }
    }

    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + 1));
}

fn cut_isolated_tile_for_meld(
    hand: &TileCounts,
    original: &TileCounts,
    num_blocks: &NumBlocks,
    min_shanten: &mut i8,
) {
    for i in 0..NUM_TILE_TYPE {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            *min_shanten = *min_shanten.min(&mut num_blocks.formula());
            return;
        }
    }

    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + 1));
}

fn cut_isolated_tile_for_pair_and_meld(
    hand: &TileCounts,
    original: &TileCounts,
    num_blocks: &NumBlocks,
    min_shanten: &mut i8,
) {
    let mut count = 0i8;

    for i in 0..NUM_TILE_TYPE {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            count += 1;
        }
    }

    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + if count >= 2 { 0 } else { 1 }));
}

struct DecompFixedPruned {}

impl ShantenCalculator for DecompFixedPruned {
    fn new() -> Self {
        DecompFixedPruned {}
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_num_meld = (hand.iter().sum::<TileCount>() / 3) as i8;
        let num_call = 4 - required_num_meld;
        let mut num_blocks = NumBlocks {
            num_meld: num_call,
            num_meld_cand: 0,
            num_pair: 0,
        };
        let mut hand_clone = *hand;

        let mut min_shanten = MAX_SHANTEN;

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPE {
            if hand_clone[i] >= 2 {
                num_blocks.num_pair += 1;
                hand_clone[i] -= 2;
                cut_meld(
                    &mut hand_clone,
                    &hand,
                    &mut num_blocks,
                    &mut min_shanten,
                    i,
                    0,
                );
                hand_clone[i] += 2;
                num_blocks.num_pair -= 1;
            }
        }

        // Calculate the shanten number without a pair
        cut_meld(
            &mut hand_clone,
            &hand,
            &mut num_blocks,
            &mut min_shanten,
            NUM_TILE_TYPE,
            0,
        );

        min_shanten
    }
}

shanten_tests!(DecompFixedPruned);
shanten_benches!(DecompFixedPruned);
