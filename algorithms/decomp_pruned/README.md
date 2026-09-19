# Block Decomposition — Pruned

This crate is an ablation study of [`decomp_tomohxx`](../decomp_tomohxx): it retains
lower-bound pruning but removes the correctness fixes. It minimizes the ordinary
block-decomposition score, allowing the effect of pruning to be measured against
[`decomp`](../decomp) and the effect of the fixes against `decomp_tomohxx`.
It intentionally retains known shanten underestimates.

## Core idea

The search reserves a possible head pair, extracts complete melds, and then extracts
two-tile meld candidates. It also tries no head and alternative block extractions.
A meld candidate is a pair intended to become a triplet, or two suited tiles one
or two ranks apart that can become a sequence. A complete meld contributes two
units of progress, a candidate one, and a head one; at most four melds and
candidates combined contribute to the score.

Once meld extraction ends, the meld and head counts are fixed for the candidate
subtree. Filling every remaining meld slot with a candidate gives a lower bound
on its score. If the best score already found is no greater than that bound,
further candidate extraction cannot improve it and is skipped.

The score does not check whether the required completion tiles exist within the
four-copy limit or whether leftover tiles can supply missing blocks. Pruning
preserves this score's minimum, including its known errors.

## State and invariants

The search carries:

- `hand`, a mutable copy of the remaining tile counts;
- `melds`, the extracted meld count plus the inferred number of calls;
- `meld_candidates`, the extracted two-tile candidate count;
- `pairs`, zero or one, recording the reserved head;
- `i`, the lowest tile index eligible for extraction in the current phase;
- `min_shanten`, the best score found across all head choices;
- `lower_bound`, computed when entering a candidate subtree.

Each extraction consumes available tiles and is undone before returning.
Starting indices are nondecreasing within each phase; recursion at the same index
allows repeated blocks, while advancing leaves unused tiles for the later phase.
Candidate enumeration restarts at zero for each meld decomposition and maintains
`melds + meld_candidates <= 4`.

The meld and head counts, and therefore the lower bound, remain constant throughout
one candidate subtree. The search does not retain the original counts or the head's
tile index: two remaining copies of the head's tile type may become a candidate.

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
        remove (t, t); head = 1
        cut_meld(0)
        restore (t, t); head = 0

    cut_meld(0)
    return best

cut_meld(i):
    if i == 34:
        cut_candidate(0, lower_bound = 4 - melds - head)
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
        best = min(best, 8 - 2*melds - candidates - head)
        return

    if melds + candidates < 4:
        for each available candidate starting at i, in this order:
            pair (i, i), only when hand[i] == 2
            adjacent-tile candidate (i, i+1)
            gapped-tile candidate (i, i+2)
            remove candidate; candidates += 1
            cut_candidate(i, lower_bound)
            restore candidate; candidates -= 1

    cut_candidate(i + 1, lower_bound)
```

The hand, counters, and best score are shared between recursive calls.
Both phases include the advance branch even when an extraction is possible.
The candidate-pair check is exactly `hand[i] == 2`, including when those copies
share the head's tile type.

The bound check runs on every candidate call, so finding a sufficiently low score
also prunes later siblings in that subtree. Reaching the four-block cap disables
extractions but does not itself return: the index scan continues to the terminal
score evaluation unless the bound check prunes it.

### Shanten formula

Let $m$ be the meld count including calls, $t$ the candidate count, and
$p \in \{0,1\}$ the reserved head count. The score is

```math
S = 8 - 2m - t - p, \qquad m+t \leq 4
```

The algorithm returns the minimum score without any correction. Since
$t \leq 4-m$, every descendant of a fixed meld decomposition satisfies

```math
S \geq 8 - 2m - (4-m) - p = 4 - m - p = L
```

Thus `min_shanten <= L` means no descendant can improve the current minimum.
The bound need not be attainable to justify pruning.

## Why it works

The head loop and the two extraction phases retain the block choices of the
unpruned decomposition search. Nondecreasing indices avoid permutations of the
same extraction order. The bound discards only subtrees whose scores cannot beat
the current minimum, so it preserves the result of `decomp`.

This establishes preservation of the uncorrected score, not exact shanten
calculation. For a small counterexample, consider `1111z`, for which three calls
are inferred. Extracting a triplet gives four melds and no head, so the formula
returns zero. Turning the leftover honor into a head would require a fifth copy;
the correct shanten number is one. Reserving a head instead also permits the other
two copies to be counted as a triplet candidate, again relying on a fifth copy.
The removed fixes address these invalid completion assumptions.

## Complexity

Let $T=34$ be the tile-type count and $n$ the input tile count. There are at most
$T+1$ head choices, each enumerating meld decompositions and their candidate
subtrees. This nested search is combinatorial, with a loose exponential time
bound in $T+n$ per head choice. Lower-bound pruning reduces the number of visited
branches but does not improve that worst-case bound.

An extraction removes at least two tiles and each phase advances through at most
$T$ indices, giving recursion depth $O(T+n)$. Auxiliary space is $O(T+n)$ for the
hand copy and recursion stack. Each bound check and terminal score evaluation is
constant time; there is no table construction or memoization.

## Implementation notes

[`cut_meld` and `cut_meld_cand`](src/lib.rs) mutate one hand copy and backtrack
without per-branch heap allocation. Counters and scores use `i8`; the calculator
has no persistent state or cache. Melds are tried as triplets before sequences;
candidates are tried as pairs, adjacent tiles, then gapped tiles. This order affects
when a good score is found and therefore how much work the bound eliminates.

Unlike the corrected variant, terminal evaluation does not scan remaining tiles,
and recursive calls carry neither original tile counts nor a head index. Benchmark
comparisons with `decomp_tomohxx` therefore include both the removed correction
work and the search changes caused by allowing same-type head and candidate pairs
and retaining uncorrected scores.

## Correctness and limitations

- Exactness: intentionally inexact. It uses the `shanten_tests!` profile
  `legacy_decomposition` and does not pass the shared exactness suite without
  ignored cases. These failures are retained to study pruning separately from the
  correctness fixes.
- Known incorrect cases: insufficient-block hands and decompositions that assume
  an unavailable fifth copy can produce underestimates. For `1111z`, the expected
  value is one and this implementation returns zero. A hand has insufficient blocks
  when fewer than five blocks can be extracted, counting isolated tiles as blocks.

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
