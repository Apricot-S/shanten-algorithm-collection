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

fn cut_meld(hand: &mut TileCounts, num_blocks: &mut NumBlocks, min_shanten: &mut i8, i: usize) {
    if i >= NUM_TILE_TYPES {
        let lower_bound = num_blocks.calculate_lower_bound();
        cut_meld_cand(hand, num_blocks, min_shanten, 0, lower_bound);
        return;
    }

    // triplet
    if hand[i] >= 3 {
        num_blocks.num_meld += 1;
        hand[i] -= 3;
        cut_meld(hand, num_blocks, min_shanten, i);
        hand[i] += 3;
        num_blocks.num_meld -= 1;
    }

    // sequence
    if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 1] > 0 && hand[i + 2] > 0 {
        num_blocks.num_meld += 1;
        hand[i] -= 1;
        hand[i + 1] -= 1;
        hand[i + 2] -= 1;
        cut_meld(hand, num_blocks, min_shanten, i);
        hand[i + 2] += 1;
        hand[i + 1] += 1;
        hand[i] += 1;
        num_blocks.num_meld -= 1;
    }

    cut_meld(hand, num_blocks, min_shanten, i + 1);
}

fn cut_meld_cand(
    hand: &mut TileCounts,
    num_blocks: &mut NumBlocks,
    min_shanten: &mut i8,
    i: usize,
    lower_bound: i8,
) {
    // pruning
    if *min_shanten <= lower_bound {
        return;
    }

    if i >= NUM_TILE_TYPES {
        *min_shanten = *min_shanten.min(&mut num_blocks.formula());
        return;
    }

    if num_blocks.get_num_blocks() < MAX_NUM_BLOCKS {
        // pair (triplet candidate)
        if hand[i] == 2 {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 2;
            cut_meld_cand(hand, num_blocks, min_shanten, i, lower_bound);
            hand[i] += 2;
            num_blocks.num_meld_cand -= 1;
        }

        // edge joint or open joint
        if is_suit(i) && i % 9 < 8 && hand[i] > 0 && hand[i + 1] > 0 {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 1;
            hand[i + 1] -= 1;
            cut_meld_cand(hand, num_blocks, min_shanten, i, lower_bound);
            hand[i + 1] += 1;
            hand[i] += 1;
            num_blocks.num_meld_cand -= 1;
        }

        // middle joint
        if is_suit(i) && i % 9 < 7 && hand[i] > 0 && hand[i + 2] > 0 {
            num_blocks.num_meld_cand += 1;
            hand[i] -= 1;
            hand[i + 2] -= 1;
            cut_meld_cand(hand, num_blocks, min_shanten, i, lower_bound);
            hand[i + 2] += 1;
            hand[i] += 1;
            num_blocks.num_meld_cand -= 1;
        }
    }

    cut_meld_cand(hand, num_blocks, min_shanten, i + 1, lower_bound);
}

/// Block-decomposition algorithm with lower-bound pruning.
pub struct DecompPruned;

impl ShantenCalculator for DecompPruned {
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
                cut_meld(&mut hand_clone, &mut num_blocks, &mut min_shanten, 0);
                hand_clone[i] += 2;
                num_blocks.pairs -= 1;
            }
        }

        // Calculate the shanten number without a pair
        cut_meld(&mut hand_clone, &mut num_blocks, &mut min_shanten, 0);

        min_shanten
    }
}

common::shanten_tests!(
    DecompPruned,
    profile = legacy_decomposition,
    reason = "the original algorithm does not correct for insufficient isolated tiles"
);
