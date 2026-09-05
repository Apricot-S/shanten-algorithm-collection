# Block Decomposition

This algorithm estimates shanten number by removing a possible head pair, complete melds,
and two-tile meld candidates from the input hand. It exhaustively searches these
block choices and minimizes a count-based formula. It can return an incorrect
shanten number when the remaining tiles cannot supply a legal head.

## Core idea

A complete meld contributes two units of progress toward a winning hand, a
two-tile meld candidate contributes one, and a head pair contributes one. A meld
candidate is a pair intended to become a triplet, or two suited tiles separated
by one or two ranks that can become a sequence.

The search first reserves a head pair, also considering the absence of a head.
It then removes complete melds and finally meld candidates. Trying alternative
extractions accounts for tiles that could participate in different blocks. At
most four melds and meld candidates combined contribute to the score.

This model does not track whether leftover single tiles can actually become a
head. Consequently, minimizing the block formula does not always give the exact
shanten number.

## State and invariants

The search carries:

- `hand`, a mutable copy of the remaining tile counts;
- `num_meld`, the number of extracted melds plus the inferred number of calls;
- `num_meld_cand`, the number of extracted two-tile meld candidates;
- `pairs`, either zero or one, recording the reserved head;
- `i`, the lowest tile index still eligible for extraction in the current phase;
- `min_shanten`, the lowest formula value found so far.

Each extraction consumes only available tiles, updates its block counter, and
restores both counts and counter after recursion.

Within each phase, starting tile indices are nondecreasing. After an extraction,
the same index remains eligible, allowing repeated sequences and multiple blocks
starting at one tile. Advancing the index leaves any remaining copies available
to the later phase. Meld-candidate enumeration restarts at index zero for every
meld decomposition.

The head is counted separately from meld candidates. The search maintains
`num_meld + num_meld_cand <= 4`; meld extraction respects this through the number
of available tiles, while candidate extraction checks the limit explicitly.

## Algorithm

Let `calls = 4 - floor(sum(hand) / 3)`. These calls are included in the meld
counter from the start.

```text
calculate(input):
    hand = copy of input
    melds = 4 - floor(sum(input) / 3)
    candidates = 0
    head = 0
    best = 8

    for each tile type t with hand[t] >= 2:
        remove two copies of t; head = 1
        cut_meld(0)
        restore two copies of t; head = 0

    cut_meld(0)
    return best

cut_meld(i):
    if i == 34:
        cut_candidate(0)
        return

    for each available block starting at i, in this order:
        triplet (i, i, i)
        sequence (i, i+1, i+2)
        remove block; melds += 1
        cut_meld(i)
        restore block; melds -= 1

    cut_meld(i + 1)

cut_candidate(i):
    if i == 34:
        best = min(best, 8 - 2*melds - candidates - head)
        return

    if melds + candidates < 4:
        for each available block starting at i, in this order:
            pair (i, i), only when hand[i] == 2
            adjacent-tile candidate (i, i+1)
            gapped-tile candidate (i, i+2)
            remove block; candidates += 1
            cut_candidate(i)
            restore block; candidates -= 1

    cut_candidate(i + 1)
```

The pseudocode shares the mutable hand, counters, and best score between calls.
The candidate-pair condition is exactly `hand[i] == 2`, unlike the initial head
selection's `hand[i] >= 2`. Both phases always include the branch that advances
without extracting a block. Reaching the four-block limit disables further
candidate extraction but does not stop the index scan. There is no pruning based
on the current best score.

### Shanten formula

Let $m$ be the number of extracted melds, $c$ the inferred number of calls, $t$ the
number of meld candidates, and $p \in \{0,1\}$ the number of reserved head pairs.
The candidate score is

```math
S = 8 - 2(m+c) - t - p,
\qquad m+c+t \leq 4
```

Equivalently, with $k = 4-c$, the score is $2k - 2m - t - p$. Each complete meld
supplies two units of progress, each candidate supplies one, and the head supplies
one. The implementation stores $m+c$ together in `num_meld` and returns the minimum
score over the enumerated decompositions.

## Why it works

Reserving each possible head and also trying no head covers the head choices.
Within each extraction phase, the recursive choices consider competing uses of
the same tiles; the advance branch allows blocks to be omitted. Sorting blocks
by their starting index preserves the represented block choices without requiring
all permutations of their extraction order. The candidate-count cap prevents
crediting more than four meld slots.

These properties explain the search for a minimum block score, but they do not
prove exact shanten calculation. A physically available decomposition need not
admit the completion assumed by its score: the algorithm never checks the tiles
needed to complete a target or whether a leftover tile can form its head.

For a small counterexample, consider `1111z`. The algorithm infers three calls,
extracts a triplet, and leaves one copy of the same honor. With four melds and no
head, the formula gives zero. Completing that leftover tile into a pair would
require a fifth copy of the honor, so the actual shanten number is one. Exhaustive
block search cannot repair this missing legality condition.

## Complexity

Let $T = 34$ be the number of tile types and $n$ the number of input tiles. There
are at most $T+1$ head choices. Each launches a meld search, and every terminal
meld decomposition launches a separate candidate search over the remaining tiles.
The dominant cost is this nested enumeration of competing block decompositions;
its search space grows combinatorially with the available extractions.

Every extraction removes at least two tiles, and every advance increases the
current phase's index. A recursion path therefore has depth $O(T+n)$. Branching
is bounded by a constant, giving a loose exponential time bound in $T+n$ per head
choice. The fixed tile universe, limited hand size, available tile counts, and
four-block cap constrain the actual search. There is no
memoization or score-bound pruning to avoid repeated subproblems.

Auxiliary space is $O(T+n)$ for the copied hand and recursion stack. No tables are
constructed and no heap allocation is performed.

## Implementation notes

[`cut_meld` and `cut_meld_cand`](src/lib.rs) implement separate recursive phases
over one shared mutable hand. They update counts in place and backtrack instead
of allocating a new hand for every branch. Block counters and scores use `i8`.

The calculator is a unit struct with no persistent state or cache. It copies the
input once per calculation. Even when the block cap is reached, the candidate
phase scans to the end before evaluating the formula; this crate retains that
baseline behavior for comparison with the pruned variants.

## Correctness and limitations

- Exactness: not theoretically exact and does not pass the shared exactness suite
  without ignored cases. It uses the `shanten_tests!` profile
  `legacy_decomposition`, preserving the historical behavior for benchmarking
  against corrected implementations. The shared suite has 16 passing tests and
  16 ignored known failures; explicitly running the ignored tests reproduces all
  16 failures.
- Known incorrect cases include insufficient usable isolated tiles, fifth-copy
  waits, and incomplete hands containing all required melds but no head. Examples
  from the shared suite are:

| Hand               | Expected | Actual | Limitation                                                      |
| ------------------ | -------: | -----: | --------------------------------------------------------------- |
| `1111z`            |        1 |      0 | The leftover honor cannot form a legal head.                    |
| `1111222333444z`   |        1 |      0 | Four melds leave only an unusable fourth honor.                 |
| `11m111122223333z` |        2 |      1 | The block score assumes more usable isolated tiles than exist.  |
| `234p567s`         |        1 |      0 | All required melds are present, but no tile remains for a head. |

The isolated-tile limitation is examined in
[ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## Origin and references

- Algorithm source credited by this crate:
  [麻雀C言語プログラム集, archived June 16, 2019](https://web.archive.org/web/20190616213620/http://cmj3.web.fc2.com/).
- Additional reference:
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

### Differences from the source

This Rust implementation uses the source's head-first, meld-then-candidate
decomposition approach and retains its isolated-tile limitation. It derives the
number of calls from the input tile count and implements backtracking with a
fixed-size tile-count array. The repository does not pin an upstream source file
or revision beyond the archived site reference, so source-level fidelity is not
established.

## License

The original page states, "無断使用、改造自由(ライセンスフリー)です" (the source may be
used and modified freely without permission).
