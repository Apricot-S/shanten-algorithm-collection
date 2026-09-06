use common::{NUM_TILE_TYPES, ShantenCalculator, TileCount, TileCounts};

const MAX_NUM_BLOCKS: i8 = 4;
const MAX_SHANTEN: i8 = 8;

struct BlockCounts {
    melds: i8,
    meld_candidates: i8,
    pairs: i8,
}

impl BlockCounts {
    fn total_meld_blocks(&self) -> i8 {
        self.melds + self.meld_candidates
    }

    fn shanten_number(&self) -> i8 {
        MAX_SHANTEN - self.melds * 2 - self.meld_candidates - self.pairs
    }

    fn lower_bound(&self) -> i8 {
        MAX_NUM_BLOCKS - self.melds - self.pairs
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
    block_counts: &mut BlockCounts,
    min_shanten: &mut i8,
    pair_index: usize,
    i: usize,
) {
    if i >= NUM_TILE_TYPES {
        let lower_bound = block_counts.lower_bound();
        cut_meld_cand(
            hand,
            original,
            block_counts,
            min_shanten,
            pair_index,
            0,
            lower_bound,
        );
        return;
    }

    // triplet
    if hand[i] >= 3 {
        block_counts.melds += 1;
        hand[i] -= 3;
        cut_meld(hand, original, block_counts, min_shanten, pair_index, i);
        hand[i] += 3;
        block_counts.melds -= 1;
    }

    // sequence
    if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 1] > 0 && hand[i + 2] > 0 {
        block_counts.melds += 1;
        hand[i] -= 1;
        hand[i + 1] -= 1;
        hand[i + 2] -= 1;
        cut_meld(hand, original, block_counts, min_shanten, pair_index, i);
        hand[i + 2] += 1;
        hand[i + 1] += 1;
        hand[i] += 1;
        block_counts.melds -= 1;
    }

    cut_meld(hand, original, block_counts, min_shanten, pair_index, i + 1);
}

fn cut_meld_cand(
    hand: &mut TileCounts,
    original: &TileCounts,
    block_counts: &mut BlockCounts,
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
        if (block_counts.melds == 4 && block_counts.meld_candidates == 0 && block_counts.pairs == 0)
            || (block_counts.melds == 3
                && block_counts.meld_candidates == 1
                && block_counts.pairs == 0)
        {
            // lack of isolated tiles pattern 1: isolated tile for pair
            cut_isolated_tile_for_pair(hand, original, block_counts, min_shanten);
            return;
        } else if block_counts.melds == 3
            && block_counts.meld_candidates == 0
            && block_counts.pairs == 1
        {
            // lack of isolated tiles pattern 2: isolated tile for meld
            cut_isolated_tile_for_meld(hand, original, block_counts, min_shanten);
            return;
        } else if block_counts.melds == 3
            && block_counts.meld_candidates == 0
            && block_counts.pairs == 0
        {
            // lack of isolated tiles pattern 3: isolated tile 1 for pair, isolated tile 2 for meld
            cut_isolated_tile_for_pair_and_meld(hand, original, block_counts, min_shanten);
            return;
        }
        // enough isolated tiles
        *min_shanten = *min_shanten.min(&mut block_counts.shanten_number());
        return;
    }

    if block_counts.total_meld_blocks() < MAX_NUM_BLOCKS {
        // pair (triplet candidate)
        if hand[i] == 2 && i != pair_index {
            block_counts.meld_candidates += 1;
            hand[i] -= 2;
            cut_meld_cand(
                hand,
                original,
                block_counts,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i] += 2;
            block_counts.meld_candidates -= 1;
        }

        // edge joint or open joint
        if is_suit(i) && i % 9 < 8 && hand[i] > 0 && hand[i + 1] > 0 {
            block_counts.meld_candidates += 1;
            hand[i] -= 1;
            hand[i + 1] -= 1;
            cut_meld_cand(
                hand,
                original,
                block_counts,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i + 1] += 1;
            hand[i] += 1;
            block_counts.meld_candidates -= 1;
        }

        // middle joint
        if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 2] > 0 {
            block_counts.meld_candidates += 1;
            hand[i] -= 1;
            hand[i + 2] -= 1;
            cut_meld_cand(
                hand,
                original,
                block_counts,
                min_shanten,
                pair_index,
                i,
                lower_bound,
            );
            hand[i + 2] += 1;
            hand[i] += 1;
            block_counts.meld_candidates -= 1;
        }
    }

    cut_meld_cand(
        hand,
        original,
        block_counts,
        min_shanten,
        pair_index,
        i + 1,
        lower_bound,
    );
}

fn cut_isolated_tile_for_pair(
    hand: &TileCounts,
    original: &TileCounts,
    block_counts: &BlockCounts,
    min_shanten: &mut i8,
) {
    for i in 0..NUM_TILE_TYPES {
        if hand[i] > 0 && original[i] < 3 {
            // enough isolated tiles
            *min_shanten = *min_shanten.min(&mut block_counts.shanten_number());
            return;
        }
    }

    // lack of isolated tiles
    *min_shanten = *min_shanten.min(&mut (block_counts.shanten_number() + 1));
}

fn cut_isolated_tile_for_meld(
    hand: &TileCounts,
    original: &TileCounts,
    block_counts: &BlockCounts,
    min_shanten: &mut i8,
) {
    for i in 0..NUM_TILE_TYPES {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            // enough isolated tiles
            *min_shanten = *min_shanten.min(&mut block_counts.shanten_number());
            return;
        }
    }

    // lack of isolated tiles
    *min_shanten = *min_shanten.min(&mut (block_counts.shanten_number() + 1));
}

fn cut_isolated_tile_for_pair_and_meld(
    hand: &TileCounts,
    original: &TileCounts,
    block_counts: &BlockCounts,
    min_shanten: &mut i8,
) {
    let mut count = 0i8;

    for i in 0..NUM_TILE_TYPES {
        if (is_suit(i) && hand[i] > 0) || (is_honor(i) && hand[i] > 0 && original[i] < 3) {
            count += 1;
        }
    }

    *min_shanten = *min_shanten.min(&mut (block_counts.shanten_number() + i8::from(count < 2)));
}

/// Corrected block-decomposition algorithm with lower-bound pruning.
pub struct DecompFixedPruned;

impl ShantenCalculator for DecompFixedPruned {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_melds = (hand.iter().sum::<TileCount>() / 3).cast_signed();
        let called_melds = 4 - required_melds;
        let mut block_counts = BlockCounts {
            melds: called_melds,
            meld_candidates: 0,
            pairs: 0,
        };
        let mut hand_clone = *hand;

        let mut min_shanten = MAX_SHANTEN;

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPES {
            if hand_clone[i] >= 2 {
                block_counts.pairs += 1;
                hand_clone[i] -= 2;
                cut_meld(
                    &mut hand_clone,
                    hand,
                    &mut block_counts,
                    &mut min_shanten,
                    i,
                    0,
                );
                hand_clone[i] += 2;
                block_counts.pairs -= 1;
            }
        }

        // Calculate the shanten number without a pair
        cut_meld(
            &mut hand_clone,
            hand,
            &mut block_counts,
            &mut min_shanten,
            NUM_TILE_TYPES,
            0,
        );

        min_shanten
    }
}

common::shanten_tests!(DecompFixedPruned);
