# Block Decomposition — Kobayashi

This algorithm estimates shanten number by reserving a possible head pair and
summarizing each suit with two representative block counts. It counts meld
candidates and isolated tiles from groups of remaining tiles instead of searching
for individual candidates. Its score accounts for missing blocks, but the loss of
tile identities still permits incorrect results when a completion needs a fifth copy.

## Core idea

A meld candidate is a pair intended to become a triplet, or two suited tiles one
or two ranks apart that can become a sequence. An isolated tile is a one-tile
starting point for a missing block. A complete meld retains three tiles toward a
winning shape, a candidate or head retains two, and an isolated tile retains one.
The score credits these tiles subject to four meld slots and one head slot.

The search reserves each possible head and also considers the absence of a head.
It enumerates complete melds independently in each suit, as in
[`decomp_ara`](../decomp_ara). Instead of extracting candidates recursively, it
groups the remaining tiles at gaps of at least two consecutive empty ranks. For a group of
$k$ tiles, it records $\lfloor k/2\rfloor$ candidates and $k\bmod2$ isolated tiles.
For example, `2244589p` splits into `22445p` and `89p`, giving three candidates
and one isolated tile. These are numerical summaries; no actual candidate
pairing or isolated-tile identity is retained.

Each suit keeps two patterns with meld count $m$, candidate count $t$, and
isolated-tile count $i$:

- A minimizes $i$, breaking ties by minimizing $t$.
- B maximizes $m$, breaking ties by maximizing $t$.

A favors using tiles in larger blocks, while B prioritizes complete melds when
excess blocks must be discarded. Honors contribute fixed counts. The algorithm
scores all eight combinations of A or B across the three suits.

## State and invariants

The search carries:

- `hand`, a mutable copy of the remaining tile counts;
- `has_pair`, recording whether a head has been reserved;
- `called_melds`, the inferred number of calls;
- `n`, the lowest rank still eligible for meld extraction in the current suit;
- `BlockCountPatterns`, the A and B summaries, each holding `melds`,
  `meld_candidates`, and `isolated`;
- `min`, the lowest formula value found across the combined patterns or head choices.

Every meld or head extraction consumes available tiles and restores them before
returning. Starting ranks are nondecreasing. A sequence recurses at the same rank
to allow repeated sequences; a triplet advances to the next rank because at most
one copy remains. The advance branch leaves tiles available for the terminal
group-counting scan.

Calls are inferred before removing a head and remain fixed for all head choices.
The reserved head is separate from suit and honor counts. Slot limits are applied
only after combining those counts with calls. A and B are updated independently
on strict improvement, so complete ties retain the first pattern encountered.

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

    honors = (0, 0, 0)
    for each honor type:
        if count >= 3: add one meld to honors
        if count == 2: add one candidate to honors
        if count == 1: add one isolated tile to honors

    best = 13
    for each of the eight combinations of suit patterns A or B:
        (M, T, I) = (calls, 0, 0) + honors + sum(selected suit counts)
        best = min(best, score(M, T, I, head))

    return best

melds(suit, rank):
    if rank == 9:
        return remaining_groups(suit)
    best = melds(suit, rank + 1)

    for each available block starting at rank, in this order:
        sequence (rank, rank+1, rank+2)
        triplet (rank, rank, rank)
        remove block
        next_rank = rank for a sequence, rank + 1 for a triplet
        result = melds(suit, next_rank)
        restore block
        add one meld to both patterns in result
        merge(best, result)

    return best

remaining_groups(suit):
    candidates = isolated = 0
    split remaining tiles at gaps of two or more empty ranks

    for each group with k tiles:
        candidates += floor(k / 2)
        isolated += k mod 2

    return A=(0, candidates, isolated), B=(0, candidates, isolated)

merge(best, result):
    replace best.A if (result.A.isolated, result.A.candidates)
        is lexicographically smaller
    replace best.B if (result.B.melds, result.B.candidates)
        is lexicographically larger
```

Ranks are zero-based, and blocks must stay within their suit. The advance branch
is tried before either extraction. An honor quad contributes one meld and no
isolated tile: the fourth copy cannot be paired without a fifth copy.
Suited leftovers are counted numerically, even
when they are fourth copies of tiles used by extracted melds. There is no pruning
based on the current best score and no memoization.

### Shanten formula

For a combined pattern with $m$ melds including calls, $t$ meld candidates,
$i$ isolated tiles, and $p\in\{0,1\}$ reserved heads, first downgrade excess
candidates to isolated tiles, then discard excess isolated tiles:

```math
t' = \min(t,4-m),\qquad
i' = \min\bigl(i+t-t',\;5-p-m-t'\bigr).
```

The candidate score is

```math
S = 13 - 3m - 2t' - i' - 2p,
\qquad m\leq4.
```

The caps enforce $m+t'\leq4$ and $m+t'+i'+p\leq5$. The scoring helper also
handles $m>4$ by adding $m-4$ to the candidate count and setting $m=4$ before
these adjustments. Available tiles and the inferred call count keep $m\leq4$
for the supported inputs.

This is the number of missing tiles in a fourteen-tile target, minus one, under
the block-count model. Unlike a score based only on melds and candidates, it
penalizes missing starting tiles: four melds with neither a head nor an isolated
tile score one, rather than zero.

## Why it works

Trying every available pair and the no-head case covers the possible reserved
heads. Meld extraction enumerates competing choices without sharing tiles;
nondecreasing starting ranks avoid permutations of the same extraction. Adding
one meld to both returned patterns preserves each pattern's comparison order.
Thus the recursion computes the stated A and B summaries of its group counts.

A gap of two empty ranks prevents a candidate from connecting tiles across the
gap. Group counting uses this separation to summarize leftovers without a second
recursive extraction phase. However, group size and parity do not retain the
tile identities needed to check legal completions. Neither this summary nor the
slot adjustments establish exact shanten calculation.

For a small counterexample, `1111z` infers three calls. Without a head, the honor
quad contributes one meld and no isolated tile, giving a score of one. Reserving
`11z` as the head instead leaves `11z` as a candidate. With three calls, one
candidate, and one head, the formula gives zero. Completing that candidate would
require a fifth copy of the honor, so the correct result is one. The minimum over
head choices therefore still produces an incorrect result.

## Complexity

Let $T=34$ be the tile-type count, $R=9$ the ranks per suit, and $n$ the input tile
count. There are at most $T+1$ head choices, each launching three suit searches.
The dominant cost is the enumeration of complete meld decompositions. Each leaf
scans $R$ ranks to count remaining groups, replacing the recursive candidate
search used by `decomp_ara`.

Each path advances through at most $R$ ranks and extracts at most $n/3$ melds.
Constant branching gives a loose exponential time bound in $R+n$ per suit per
head choice, with an additional $O(R)$ scan per leaf. Honor counting scans seven
types, and combination work evaluates eight patterns. The fixed tile universe
and limited hand size constrain the practical search space.

Auxiliary space is $O(T+R+n)$ for the copied hand and recursion stack. There is no
one-time table construction, heap allocation, or persistent cache.

## Implementation notes

[`count_suit_blocks` and `count_meld_candidates`](src/lib.rs) operate on
nine-element slices. The latter scans the remaining counts without modifying them.
Each recursive result contains two triples of `i8` counters.

The calculator is a unit struct and copies the input once per calculation. Every
head choice recomputes all three suit summaries and honor counts, even when only
one suit changes. Equal A and B patterns still participate in all eight
combinations.

## Correctness and limitations

- Exactness: not theoretically exact and does not pass the shared exactness
  suite without ignored cases. It uses the `shanten_tests!` ignore reason
  `"insufficient_isolated_tiles"`, preserving the historical behavior for
  benchmarking against corrected implementations. The four-meld, no-pair case
  passes without the `"incomplete_hand"` ignore reason.
- Known incorrect cases: hands with insufficient blocks. A hand has insufficient
  blocks when fewer than five blocks can be taken from it, counting isolated tiles
  as blocks for this definition. This limitation is examined in
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## Origin and references

- Devised by: [Satoshi Kobayashi](https://github.com/kobalab).
- Primary source:
  [kobalab/majiang-core, `lib/xiangting.js`](https://github.com/kobalab/majiang-core/blob/master/lib/xiangting.js).
  The original port's upstream version or commit is not recorded in this crate.
- Additional reference:
  [対戦型麻雀ゲームAIのアルゴリズムと実装](https://www.amazon.co.jp/dp/4798067881).
- Additional reference:
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

### Differences from the source

- Calls are inferred from the input tile count instead of an explicit meld list.
- The upstream game-state check that changes a winning score to zero during a
  call before the discard is omitted; the calculator has no corresponding state.
