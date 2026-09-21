# Block Decomposition — Ara

This algorithm estimates shanten number by reserving a possible head pair and
decomposing each suit independently into melds and two-tile meld candidates. It
retains two representative block counts per suit and evaluates their eight
combinations. The count-based score can be incorrect when the remaining tiles
cannot supply a legal head.

## Core idea

A meld candidate is a pair intended to become a triplet, or two suited tiles one
or two ranks apart that can become a sequence. A complete meld contributes two
units of progress, a candidate contributes one, and a reserved head contributes
one. Only four meld slots can contribute to the score.

The search reserves each possible head and also considers the absence of a head.
For each suit, it extracts complete melds and then meld candidates, as in
[`decomp`](../decomp). It keeps two patterns, each storing meld count $m$ and
candidate count $t$:

- A maximizes $2m+t$, favoring progress before the four-slot limit is applied.
- B maximizes $10m+t$, prioritizing complete melds and then candidates. Within
  the supported hand size, ten makes one extra meld outweigh any possible
  candidate-count difference.

The second pattern matters because excess candidates are discarded when the
suits are combined. A pattern with more complete melds can then be better than
one with a higher uncapped score. Honors contribute fixed counts, and the search
scores all $2^3$ choices of A or B across the suits for every head choice.

## State and invariants

The search carries:

- `hand`, a mutable copy of the remaining tile counts;
- `has_pair`, recording whether a head has been reserved;
- `called_melds`, the inferred number of calls;
- `n`, the lowest rank still eligible for extraction in the current suit and phase;
- `BlockCountPatterns`, the A and B summaries, each holding `melds` and
  `meld_candidates`;
- `min`, the lowest formula value found across the combined patterns or head choices.

Every extraction consumes available tiles and restores them before returning.
Starting ranks are nondecreasing within each phase. Extraction recurses at the
same rank, allowing repeated sequences and multiple blocks starting at one tile.
Advancing leaves remaining copies available to the later phase. Candidate
enumeration restarts at rank zero for every meld decomposition.

Calls are inferred before removing a head and remain unchanged throughout the
calculation. The head is counted separately from candidates. Suit summaries retain
only block counts, not tile identities or isolated-tile counts.

Suit searches do not enforce the four-slot limit. It is applied only after
combining the suit and honor counts. A and B are updated independently using
strict improvement, so ties retain the first pattern encountered.

## Algorithm

Let `calls = 4 - floor(sum(hand) / 3)`. Calls are added to the meld count when
combining the three suit summaries with the honor counts.

```text
calculate(input):
    hand = copy of input
    calls = 4 - floor(sum(input) / 3)
    best = evaluate(hand, head=0, calls)

    for each tile type with at least two copies:
        remove two copies
        best = min(best, evaluate(hand, head=1, calls))
        restore two copies

    return best

evaluate(hand, head, calls):
    for each suit:
        patterns[suit] = melds(suit, 0)

    honor_melds = number of honor types with at least three copies
    honor_candidates = number of honor types with exactly two copies
    best = 8

    for each of the eight combinations of suit patterns A or B:
        M = calls + honor_melds + sum(selected suit meld counts)
        T = honor_candidates + sum(selected suit candidate counts)
        best = min(best, score(M, T, head))

    return best

melds(suit, rank):
    if rank == 9:
        return candidates(suit, 0)
    best = melds(suit, rank + 1)

    for each available block starting at rank, in this order:
        sequence (rank, rank+1, rank+2)
        triplet (rank, rank, rank)
        remove block
        result = melds(suit, rank)
        restore block
        add one meld to both patterns in result
        merge(best, result)

    return best

candidates(suit, rank):
    if rank == 9:
        return A=(0, 0), B=(0, 0)
    best = candidates(suit, rank + 1)

    for each available block starting at rank, in this order:
        adjacent tiles (rank, rank+1)
        gapped tiles (rank, rank+2)
        pair (rank, rank), only when suit[rank] == 2
        remove block
        result = candidates(suit, rank)
        restore block
        add one candidate to both patterns in result
        merge(best, result)

    return best

merge(best, result):
    replace best.A if result.A has strictly greater 2*melds + candidates
    replace best.B if result.B has strictly greater 10*melds + candidates
```

Ranks are zero-based, and blocks must stay within their suit. Both phases first
try advancing without extraction, leaving those tiles available to the later
phase. The candidate-pair condition is exactly two copies, unlike the head
condition of at least two copies. An honor quad contributes one meld; its extra
tile contributes nothing. There is no pruning based on the current best score
and no memoization.

### Shanten formula

For a combined pattern with $m$ melds including calls, $t$ meld candidates, and
$p\in\{0,1\}$ reserved heads, the candidate score is

```math
S = 8 - 2m - \min(t,4-m) - p,
\qquad m \leq 4
```

The candidate cap prevents crediting more than four meld slots. It is applied
after combining suits, rather than during candidate extraction as in `decomp`.
The head credit is added separately and does not consume a meld slot. The minimum
score is taken over all eight combinations and every head choice.

The scoring helper also handles $m>4$ by adding $m-4$ to the candidate count and
setting $m=4$ before applying the cap. Available tiles and the inferred call count
keep $m\leq4$ for the supported inputs.

## Why it works

Trying every available head pair and the no-head case represents the head
choices. Within each suit, removal and restoration prevent blocks from sharing
tiles. Nondecreasing starting ranks avoid enumerating permutations of the same
extractions, while the advance branch allows competing block choices to be
considered. The A and B objectives are additive, so adding a fixed block to a
recursive result preserves the ordering for each objective.

These properties explain how the search computes its two suit summaries and
scores their combinations; they do not establish exact shanten calculation.
A count-only summary does not verify whether a completion needs a fifth copy
of a tile or whether a usable tile remains for the head.

For a small counterexample, `1111z` infers three calls and contributes one honor
meld. With four melds and no head, the score is zero. The leftover honor cannot
be paired without a fifth copy, so the correct shanten number is one. Keeping
both A and B cannot correct this missing legality condition.

## Complexity

Let $T=34$ be the number of tile types, $R=9$ the ranks per suit, and $n$ the
input tile count. There are at most $T+1$ head choices. Each launches three independent
suit searches. Every terminal meld decomposition launches a candidate search
over its remaining tiles. This nested enumeration is the dominant cost; keeping
two summaries limits combination work, not the number of recursive branches.

Each path advances through at most $2R$ ranks across the two phases, and each
extraction removes at least two tiles. Stack depth is therefore $O(R+n)$.
Constant branching gives a loose exponential time bound in $R+n$ per suit per
head choice. Honor counting scans seven types, and combining suit summaries
evaluates eight patterns. The fixed tile universe and limited hand size constrain
the practical search space. There is no memoization or score-bound pruning.

Auxiliary space is $O(T+R+n)$ for the copied hand and recursion stack. There is
no table construction, heap allocation, or persistent cache.

## Implementation notes

[`count_suit_blocks` and `count_meld_candidates`](src/lib.rs) operate in place on
nine-element slices. Each recursive result contains just two pairs of `i8`
counters. The calculator is a unit struct and copies the input once per calculation.

All suits and honors are recomputed for every head choice, even when removing
the head affects only one suit. Equal A and B patterns still participate in all
eight combinations. Isolated tiles are not removed in a preprocessing pass, and
no pre-calculated suit tables are used.

## Correctness and limitations

- Exactness: not theoretically exact and does not pass the shared exactness
  suite without ignored cases. It uses the `shanten_tests!` ignore reasons
  `"insufficient_isolated_tiles"` and `"incomplete_hand"`, preserving the historical
  behavior for benchmarking against corrected implementations.
- Known incorrect cases: hands with insufficient blocks. A hand has insufficient
  blocks when fewer than five blocks can be taken from it, counting isolated tiles
  as blocks for this definition. This limitation is examined in
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## Origin and references

- Devised by: [Ara](https://mahjong.ara.black/intro/selfintro.htm).
- Primary source:
  [向聴数を求めるアルゴリズム - あらの（一人）麻雀研究所](https://mahjong.ara.black/etc/shanten/index.htm).
- Additional reference:
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

### Differences from the source

This is a partial implementation of Ara's method, retaining per-suit A/B selection
with these omissions:

- Isolated-tile removal is omitted, leaving those tiles in the recursive suit search.
- Pre-calculated tables are omitted, so suit patterns are recomputed recursively
  for each head choice rather than looked up.
