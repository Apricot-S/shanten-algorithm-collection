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
}

fn is_suit(index: usize) -> bool {
    index < 27
}

fn cut_meld(hand: &mut TileCounts, counts: &mut BlockCounts, min_shanten: &mut i8, i: usize) {
    if i >= NUM_TILE_TYPES {
        cut_meld_cand(hand, counts, min_shanten, 0);
        return;
    }

    // triplet
    if hand[i] >= 3 {
        counts.melds += 1;
        hand[i] -= 3;
        cut_meld(hand, counts, min_shanten, i);
        hand[i] += 3;
        counts.melds -= 1;
    }

    // sequence
    if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 1] > 0 && hand[i + 2] > 0 {
        counts.melds += 1;
        hand[i] -= 1;
        hand[i + 1] -= 1;
        hand[i + 2] -= 1;
        cut_meld(hand, counts, min_shanten, i);
        hand[i + 2] += 1;
        hand[i + 1] += 1;
        hand[i] += 1;
        counts.melds -= 1;
    }

    cut_meld(hand, counts, min_shanten, i + 1);
}

fn cut_meld_cand(hand: &mut TileCounts, counts: &mut BlockCounts, min_shanten: &mut i8, i: usize) {
    if i >= NUM_TILE_TYPES {
        *min_shanten = *min_shanten.min(&mut counts.shanten_number());
        return;
    }

    if counts.total_meld_blocks() < MAX_NUM_BLOCKS {
        // pair (triplet candidate)
        if hand[i] == 2 {
            counts.meld_candidates += 1;
            hand[i] -= 2;
            cut_meld_cand(hand, counts, min_shanten, i);
            hand[i] += 2;
            counts.meld_candidates -= 1;
        }

        // edge joint or open joint
        if is_suit(i) && i % 9 < 8 && hand[i] > 0 && hand[i + 1] > 0 {
            counts.meld_candidates += 1;
            hand[i] -= 1;
            hand[i + 1] -= 1;
            cut_meld_cand(hand, counts, min_shanten, i);
            hand[i + 1] += 1;
            hand[i] += 1;
            counts.meld_candidates -= 1;
        }

        // middle joint
        if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 2] > 0 {
            counts.meld_candidates += 1;
            hand[i] -= 1;
            hand[i + 2] -= 1;
            cut_meld_cand(hand, counts, min_shanten, i);
            hand[i + 2] += 1;
            hand[i] += 1;
            counts.meld_candidates -= 1;
        }
    }

    cut_meld_cand(hand, counts, min_shanten, i + 1);
}

/// Basic block-decomposition algorithm.
pub struct Decomp;

impl ShantenCalculator for Decomp {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        let required_melds = (hand.iter().sum::<TileCount>() / 3).cast_signed();
        let called_melds = 4 - required_melds;
        let mut counts = BlockCounts {
            melds: called_melds,
            meld_candidates: 0,
            pairs: 0,
        };
        let mut hand_clone = *hand;

        let mut min_shanten = MAX_SHANTEN;

        // Remove a possible pair and calculate the shanten number with a pair
        for i in 0..NUM_TILE_TYPES {
            if hand_clone[i] >= 2 {
                counts.pairs += 1;
                hand_clone[i] -= 2;
                cut_meld(&mut hand_clone, &mut counts, &mut min_shanten, 0);
                hand_clone[i] += 2;
                counts.pairs -= 1;
            }
        }

        // Calculate the shanten number without a pair
        cut_meld(&mut hand_clone, &mut counts, &mut min_shanten, 0);

        min_shanten
    }
}

common::shanten_tests!(
    Decomp,
    profile = legacy_decomposition,
    reason = "the original algorithm does not correct for insufficient isolated tiles"
);
