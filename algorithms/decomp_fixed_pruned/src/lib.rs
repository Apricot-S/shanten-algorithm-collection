use common::{NUM_TILE_TYPES, ShantenCalculator, TileCount, TileCounts};

const MAX_NUM_BLOCKS: i8 = 4;
const MAX_SHANTEN: i8 = 8;

struct NumBlocks {
    num_meld: i8,
    num_meld_cand: i8,
    pairs: i8,
}

impl NumBlocks {
    fn get_num_blocks(&self) -> i8 {
        self.num_meld + self.num_meld_cand
    }

    fn formula(&self) -> i8 {
        MAX_SHANTEN - self.num_meld * 2 - self.num_meld_cand - self.pairs
    }

    fn calculate_lower_bound(&self) -> i8 {
        MAX_NUM_BLOCKS - self.num_meld - self.pairs
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
    if i >= NUM_TILE_TYPES {
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

    if i >= NUM_TILE_TYPES {
        if (num_blocks.num_meld == 4 && num_blocks.num_meld_cand == 0 && num_blocks.pairs == 0)
            || (num_blocks.num_meld == 3 && num_blocks.num_meld_cand == 1 && num_blocks.pairs == 0)
        {
            // lack of isolated tiles pattern 1: isolated tile for pair
            cut_isolated_tile_for_pair(hand, original, num_blocks, min_shanten);
            return;
        } else if num_blocks.num_meld == 3 && num_blocks.num_meld_cand == 0 && num_blocks.pairs == 1
        {
            // lack of isolated tiles pattern 2: isolated tile for meld
            cut_isolated_tile_for_meld(hand, original, num_blocks, min_shanten);
            return;
        } else if num_blocks.num_meld == 3 && num_blocks.num_meld_cand == 0 && num_blocks.pairs == 0
        {
            // lack of isolated tiles pattern 3: isolated tile 1 for pair, isolated tile 2 for meld
            cut_isolated_tile_for_pair_and_meld(hand, original, num_blocks, min_shanten);
            return;
        }
        // enough isolated tiles
        *min_shanten = *min_shanten.min(&mut num_blocks.formula());
        return;
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
    for i in 0..NUM_TILE_TYPES {
        if hand[i] > 0 && original[i] < 3 {
            // enough isolated tiles
            *min_shanten = *min_shanten.min(&mut num_blocks.formula());
            return;
        }
    }

    // lack of isolated tiles
    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + 1));
}

fn cut_isolated_tile_for_meld(
    hand: &TileCounts,
    original: &TileCounts,
    num_blocks: &NumBlocks,
    min_shanten: &mut i8,
) {
    for i in 0..NUM_TILE_TYPES {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            // enough isolated tiles
            *min_shanten = *min_shanten.min(&mut num_blocks.formula());
            return;
        }
    }

    // lack of isolated tiles
    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + 1));
}

fn cut_isolated_tile_for_pair_and_meld(
    hand: &TileCounts,
    original: &TileCounts,
    num_blocks: &NumBlocks,
    min_shanten: &mut i8,
) {
    let mut count = 0i8;

    for i in 0..NUM_TILE_TYPES {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            count += 1;
        }
    }

    *min_shanten = *min_shanten.min(&mut (num_blocks.formula() + i8::from(count < 2)));
}

/// Corrected block-decomposition algorithm with lower-bound pruning.
pub struct DecompFixedPruned;

impl ShantenCalculator for DecompFixedPruned {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_num_meld = (hand.iter().sum::<TileCount>() / 3).cast_signed();
        let num_call = 4 - required_num_meld;
        let mut num_blocks = NumBlocks {
            num_meld: num_call,
            num_meld_cand: 0,
            pairs: 0,
        };
        let mut hand_clone = *hand;

        let mut min_shanten = MAX_SHANTEN;

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPES {
            if hand_clone[i] >= 2 {
                num_blocks.pairs += 1;
                hand_clone[i] -= 2;
                cut_meld(
                    &mut hand_clone,
                    hand,
                    &mut num_blocks,
                    &mut min_shanten,
                    i,
                    0,
                );
                hand_clone[i] += 2;
                num_blocks.pairs -= 1;
            }
        }

        // Calculate the shanten number without a pair
        cut_meld(
            &mut hand_clone,
            hand,
            &mut num_blocks,
            &mut min_shanten,
            NUM_TILE_TYPES,
            0,
        );

        min_shanten
    }
}

common::shanten_tests!(DecompFixedPruned);
