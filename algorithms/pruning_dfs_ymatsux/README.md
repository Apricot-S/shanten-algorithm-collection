# Pruning DFS — ymatsux

This algorithm searches complete winning-hand targets. It avoids most of the
exhaustive search by pruning a partial target as soon as its distance from the input
hand cannot improve the upper bound supplied to that search call.

## Core idea

The algorithm constructs complete targets from the empty hand, then measures how
many tiles would have to be added to the input hand to reach each target.

Adding a meld or pair to a partial target can never reduce this distance. The
score of a partial target is therefore a lower bound for all of its descendants.
Once that score reaches the upper bound supplied to the current search call, the
entire branch can be discarded. A better result found while visiting its children
is passed as the upper bound of deeper calls.

## State and invariants

The depth-first search carries:

- `target`, a 34-element tile-count vector for the partial winning hand;
- `melds_left`, the number of melds still to add;
- `min_meld_id`, the first meld allowed at the current node;
- `entry_upper_bound`, the upper bound supplied when entering the current call;
- `min_shanten`, the lowest shanten number found while executing that call.

There are 55 meld types: 34 triplets and 21 sequences. Meld IDs are selected in
nondecreasing order, so each multiset of melds is visited once regardless of the
order in which its melds could have been added. A partial target is retained only
while no tile count exceeds four.

Every recursive call restores `target` before returning, and every descendant is a
component-wise extension of its ancestors. This monotonicity is the invariant used
by the pruning rule.

## Algorithm

The required number of melds is `floor(tile_count / 3)`. After selecting that many
melds, the search tries each of the 34 possible pairs and updates the upper bound.

```text
calculate(hand):
    target = empty tile-count vector
    melds_left = floor(sum(hand) / 3)
    return search(target, melds_left, first_meld = 0, entry_upper_bound = 8)

search(target, melds_left, first_meld, entry_upper_bound):
    min_shanten = entry_upper_bound

    if melds_left == 0:
        for each tile type:
            add its pair to target
            if target contains at most four copies of every tile:
                min_shanten = min(min_shanten, S(hand, target))
            remove the pair
        return min_shanten

    for each meld with ID at least first_meld:
        add the meld to target
        if target is legal and S(hand, target) < entry_upper_bound:
            min_shanten = min(min_shanten, search(
                target,
                melds_left - 1,
                first_meld = meld.id,
                entry_upper_bound = min_shanten,
            ))
        remove the meld

    return min_shanten
```

### Shanten formula

For an input hand $h$ and any partial or complete target $g$, let $h_t$ and $g_t$ be
the counts of tile type $t$. Define

```math
\begin{aligned}
D(h, g) &= \sum_{t=0}^{33} \max(g_t - h_t, 0), \\
S(h, g) &= D(h, g) - 1
\end{aligned}
```

Only missing target tiles contribute to $D(h, g)$: tiles in the input hand that are
not used by the target can be discarded during the corresponding tile exchanges.

For a complete target, $S(h, g)$ is its shanten candidate, and the minimum over all
legal complete targets is returned. For a partial target, $S(h, g)$ is a lower bound
for every complete target descended from it.

## Why it works

Every legal meld is present in the 55-entry table, every legal pair is tried at the
leaf, and nondecreasing meld IDs enumerate every meld multiset. Consequently, every
legal target with the required number of melds is represented by at least one
visited leaf.

Conversely, a leaf consists only of legal melds and one pair, and the four-copy
check rejects targets that cannot be a physical mahjong hand. Its distance is
therefore the number of additions required to reach a legal target, minus one by
the definition of shanten.

Extending a partial target can only preserve or increase each positive tile-count
deficit. Neither $D(h, g)$ nor $S(h, g)$ can decrease, so a branch whose score is at
least the upper bound supplied to the current call cannot contain a better result
than was already known when that call began. Pruning such a branch does not change
the minimum.

## Complexity

Let $M = 55$ be the number of meld types, $P = 34$ be the number of pair types,
$T = 34$ be the number of tile types, and $k \leq 4$ be the required number of melds.
Without pruning, the search has $\binom{M+d-1}{d}$ nodes at depth $d$ and tries all
$P$ pairs at depth $k$. Since legality and distance checks scan $T$ tile counts, the
worst-case time is

```math
O\left(
    T\left(
        \sum_{d=1}^{k} \binom{M+d-1}{d}
        + P\binom{M+k-1}{k}
    \right)
\right)
```

All of these parameters are bounded by the fixed mahjong tile universe. In
practice, the four-copy check and distance-bound pruning remove most candidate
subtrees.

The implementation defines the $O(M)$ meld table as a compile-time constant. Each
calculation uses $O(T + k)$ auxiliary space for the target vector and recursion
stack and performs no heap allocation.

## Implementation notes

The 34 triplets followed by the 21 sequences are defined as a compile-time constant
table. No meld-table construction occurs when a calculator is created.

The implementation updates the target vector in place when entering and leaving a
branch. It recalculates the distance by scanning all 34 tile counts at each visited
candidate. Sibling branches are checked against the bound supplied on entry to the
current call. The best result found so far is passed to recursive calls, where it
becomes the next call's entry bound.

## Correctness and limitations

- Exactness: theoretically exact because target enumeration is complete and pruning
  preserves the minimum; passes the shared exactness suite without a known-failure
  profile or ignored cases.
- Performance limitation: the search does not memoize repeated tile-count targets
  and rescans all 34 counts for each legality and distance check.

## Origin and references

- Devised by: [Yoshitake Matsumoto (ymatsux)](https://github.com/ymatsux)
- Primary source: [`ShantensuUtil.java` from MjaiClients 0.1.5](https://github.com/gimite/MjaiClients/blob/0410a2f56c9e07621c50138d27006896eb9e4962/src/org/ymatsux/mjai/client/ShantensuUtil.java)
- Additional reference: [枝刈りDFS - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/dfs/)

### Differences from the source

- The number of melds is derived from the input tile count instead of always being
  four.
- The Java collections and utility classes are replaced by a compile-time,
  fixed-size Rust table; the target-search method itself is preserved.

## License

The original MjaiClients source declares the New BSD License and names Yoshitake Matsumoto as its author.
