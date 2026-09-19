# Block Decomposition — Pruned

This crate is an ablation study of [`decomp_tomohxx`](../decomp_tomohxx): it retains
lower-bound pruning but removes the correctness fixes. It minimizes the ordinary
block-decomposition score, allowing the effect of pruning to be measured against
[`decomp`](../decomp) and the effect of the fixes against `decomp_tomohxx`.
It intentionally retains known shanten underestimates.

## Core idea

A complete meld contributes two units of progress toward a winning hand, a
two-tile meld candidate contributes one, and a head pair contributes one. A meld
candidate is a pair intended to become a triplet, or two suited tiles separated
by one or two ranks that can become a sequence.

The search first reserves a head pair, also considering the absence of a head.
It then removes complete melds and finally meld candidates. Trying alternative
extractions accounts for tiles that could participate in different blocks. At
most four melds and meld candidates combined contribute to the score.

Once meld extraction finishes, the selected meld and head counts determine
the best score that any continuation of the meld-candidate search could attain. If
the best score already found is no greater than this lower bound, the entire
candidate search for that meld decomposition is skipped.

The score does not check whether the required completion tiles exist within the
four-copy limit or whether leftover tiles can supply missing blocks. Pruning
preserves this score's minimum, including its known errors.

## State and invariants

The search carries:

- `hand`, a mutable copy of the remaining tile counts;
- `melds`, the number of extracted melds plus the inferred number of calls;
- `meld_candidates`, the number of extracted two-tile meld candidates;
- `pairs`, either zero or one, recording the reserved head;
- `i`, the lowest tile index still eligible for extraction in the current phase;
- `min_shanten`, the lowest score found so far;
- `lower_bound`, a lower bound on the score from the current meld decomposition.

Each extraction consumes only available tiles, updates its block counter, and
restores both counts and counter after recursion.

Within each phase, starting tile indices are nondecreasing. After an extraction,
the same index remains eligible, allowing repeated sequences and multiple blocks
starting at one tile. Advancing the index leaves any remaining copies available
to the later phase. Meld-candidate enumeration restarts at index zero for every
meld decomposition.

The head is counted separately from meld candidates. The search maintains
`melds + meld_candidates <= 4`; meld extraction respects this through the number
of available tiles, while candidate extraction checks the limit explicitly.

The lower bound is calculated once when the search moves from meld extraction to
meld-candidate extraction and remains valid throughout that candidate subtree.
The search does not retain the original counts or the head's tile index: two
remaining copies of the head's tile type may become a pair-shaped meld candidate.

## Algorithm

Let `called_melds = 4 - floor(sum(hand) / 3)`. The search initializes `melds` with
this value, tries each possible head, and also searches without a head.

```text
calculate(input):
    hand = copy of input
    melds = 4 - floor(sum(input) / 3)
    meld_candidates = 0
    pairs = 0
    best = 8

    for each tile type t with hand[t] >= 2:
        remove (t, t); pairs = 1
        cut_meld(0)
        restore (t, t); pairs = 0

    cut_meld(0)
    return best

cut_meld(i):
    if i == 34:
        lower_bound = 4 - melds - pairs
        cut_candidate(i = 0, lower_bound)
        return

    for each available meld starting at i, in this order:
        triplet (i, i, i)
        sequence (i, i+1, i+2)
        remove meld; melds += 1
        cut_meld(i)
        restore meld; melds -= 1

    cut_meld(i + 1)

cut_candidate(i, lower_bound):
    if best <= lower_bound:
        return

    if i == 34:
        best = min(best, 8 - 2*melds - meld_candidates - pairs)
        return

    if melds + meld_candidates < 4:
        for each available candidate starting at i, in this order:
            pair (i, i), only when hand[i] == 2
            adjacent-tile candidate (i, i+1)
            gapped-tile candidate (i, i+2)
            remove candidate; meld_candidates += 1
            cut_candidate(i, lower_bound)
            restore candidate; meld_candidates -= 1

    cut_candidate(i + 1, lower_bound)
```

The pseudocode shares the mutable hand, counters, and best score between calls.
The candidate-pair condition is exactly `hand[i] == 2`, unlike the initial head
selection's `hand[i] >= 2`. Both phases always include the branch that advances
without extracting a block. A pair-shaped meld candidate may have the same tile
index as the reserved head.

The bound check runs on every candidate call, so finding a sufficiently low score
also prunes later siblings in that subtree. Reaching the four-block limit disables
further candidate extraction but does not stop the index scan unless the bound
check prunes it.

### Shanten formula

For a decomposition with $m$ melds including calls, $t$ meld candidates, and
$p \in \{0,1\}$ reserved heads, the ordinary candidate score is

```math
S = 8 - 2m - t - p,
\qquad m+t \leq 4
```

The algorithm returns the minimum score without any correction. Since
$t \leq 4-m$, every descendant of a fixed meld decomposition satisfies

```math
S \geq 8 - 2m - (4-m) - p = 4 - m - p = L
```

Thus, if `min_shanten <= L`, no continuation can lower `min_shanten` and the subtree
can be pruned. The bound need not be attainable to justify pruning.

## Why it works

The head loop, meld search, and candidate search retain the exhaustive block choices
of the base decomposition. Nondecreasing indices remove only extraction-order
duplicates. The pruning rule follows directly from $L$: every descendant has a
score at least $L$, so pruning preserves the result of `decomp`.

These properties explain the search for a minimum block score, but they do not
prove exact shanten calculation. A physically available decomposition need not
admit the completion assumed by its score: the algorithm never checks the tiles
needed to complete a target or whether a leftover tile can form its head.

For a small counterexample, consider `1111z`. The algorithm infers three calls,
extracts a triplet, and leaves one copy of the same honor. With four melds and no
head, the formula gives zero. Completing that leftover tile into a pair would
require a fifth copy of the honor, so the actual shanten number is one. Exhaustive
block search cannot repair this missing legality condition.

Reserving a head instead also permits the other two copies to be counted as a
pair-shaped meld candidate, again relying on a fifth copy. The removed fixes address
these invalid completion assumptions.

## Complexity

Let $T=34$ be the number of tile types and $n$ the number of input tiles. There are
at most $T+1$ head choices. For each choice, the algorithm enumerates competing meld
decompositions and, unless pruned, their meld-candidate decompositions. The search
space is combinatorial; the lower bound reduces practical work but does not improve
the loose exponential worst-case time bound in $T+n$.

An extraction recurses at the same index but removes at least two tiles. Advance
calls traverse at most $T$ indices in each of the two phases. A recursion path has
depth $O(T+n)$. Each bound check and terminal score evaluation is constant time.

Auxiliary space is $O(T+n)$ for the copied hand and recursion stack. The algorithm
uses no lookup table, cache, or per-branch heap allocation.

## Implementation notes

[`cut_meld` and `cut_meld_cand`](src/lib.rs) implement separate recursive phases
over one shared mutable hand. They update counts in place and backtrack instead
of allocating a new hand for every branch. Block counters and scores use `i8`.

The calculator is a unit struct with no persistent state or cache. It copies the
input once per calculation. `cut_meld_cand` checks the lower bound before inspecting
the current index, so a pruned subtree performs no further candidate enumeration.

Melds are tried as triplets before sequences; candidates are tried as pairs,
adjacent tiles, then gapped tiles. This order affects
when a good score is found and therefore how much work the bound eliminates.

Unlike the corrected variant, terminal evaluation does not scan remaining tiles,
and recursive calls carry neither original tile counts nor a head index. Benchmark
comparisons with `decomp_tomohxx` therefore include both the removed correction
work and the search changes caused by allowing same-type head and candidate pairs
and retaining uncorrected scores.

## Correctness and limitations

- Exactness: not theoretically exact and does not pass the shared exactness suite
  without ignored cases. It uses the `shanten_tests!` ignore reasons
  `"insufficient_isolated_tiles"` and `"incomplete_hand"`, preserving the historical behavior to study pruning
  separately from the correctness fixes.
- Known incorrect cases: hands with insufficient blocks. A hand has insufficient
  blocks when fewer than five blocks can be taken from it, counting isolated tiles
  as blocks for this definition. For `1111z`, the expected value is one and this
  implementation returns zero.

## Origin and references

- Pruning and correctness fixes devised by: [tomohxx](https://github.com/tomohxx).
- Immediate source: [`decomp_tomohxx`](../decomp_tomohxx), with its fixes removed
  for this ablation study.
- Primary source:
  [正確なブロック分解方式の向聴数計算アルゴリズムの提案](https://zenn.dev/tomohxx/articles/16c0d807218d2a).
- Reference implementation:
  [`shanten.cpp` from `tomohxx/shanten-test` at commit `8dd8d41`](https://github.com/tomohxx/shanten-test/blob/8dd8d41f1179997a5ce7f979a616b551ec40c868/src/shanten.cpp).
- Additional reference:
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

### Differences from the source

- Retains the lower bound $4-m-p$ and the candidate-subtree pruning rule.
- Removes the `pair_index` exclusion, allowing two remaining copies of the head's
  tile type to be extracted as a pair-shaped candidate.
- Removes all three terminal block-deficiency checks and their one-point score
  corrections, together with the original-count state those checks require.
- Retains the local implementation's inference of called melds from the input tile
  count; the upstream C++ interface receives that count explicitly.

This is an intentional ablation, not a faithful port of the complete corrected
algorithm. Relative to `decomp`, it adds pruning while retaining the same scoring
and extraction rules.

## License

The upstream `tomohxx/shanten-test` implementation is distributed under the
[MIT License](https://github.com/tomohxx/shanten-test/blob/8dd8d41f1179997a5ce7f979a616b551ec40c868/LICENSE).
Copyright (c) 2025 [tomohxx](https://github.com/tomohxx).
