# Pruning DFS — ymatsux with Updated-Bound Pruning

This algorithm is a small improvement to the ymatsux pruning DFS. It applies every
newly improved upper bound to the remaining sibling branches immediately, avoiding
recursive calls that the original control flow would enter before rejecting their
descendants.

## Core idea

The algorithm constructs complete targets from the empty hand, then measures how
many tiles would have to be added to the input hand to reach each target.

Adding a meld or pair to a partial target can never reduce this distance. The
distance of a partial target is therefore a lower bound for all of its descendants.
Once that bound reaches the current upper bound, the entire branch can be
discarded. A better result found while visiting a child becomes the upper bound for
the remaining siblings immediately.

## State and invariants

The depth-first search carries:

- `target`, a 34-element tile-count vector for the partial winning hand;
- `num_left_meld`, the number of melds still to add;
- `min_meld_id`, the first meld allowed at the current node;
- `upper_bound`, the lowest shanten number found so far.

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
    return search(target, melds_left, first_meld = 0, upper_bound = 8)

search(target, melds_left, first_meld, upper_bound):
    if melds_left == 0:
        for each tile type:
            add its pair to target
            if target contains at most four copies of every tile:
                upper_bound = min(upper_bound, distance(hand, target) - 1)
            remove the pair
        return upper_bound

    for each meld with ID at least first_meld:
        add the meld to target
        if target is legal and distance(hand, target) - 1 < upper_bound:
            upper_bound = min(upper_bound, search(
                target,
                melds_left - 1,
                first_meld = meld.id,
                upper_bound,
            ))
        remove the meld

    return upper_bound
```

### Shanten formula

For an input hand `h` and a complete target `g`, define the missing-tile distance
as

```text
D(h, g) = sum(max(g[tile] - h[tile], 0)).
```

The shanten number associated with that target is

```text
S(h, g) = D(h, g) - 1.
```

Only missing target tiles contribute to `D`: tiles in the input hand that are not
used by the target can be discarded during the corresponding tile exchanges. The
minimum value of `S(h, g)` over all legal targets is returned.

For a partial target `g`, `D(h, g) - 1` is the lower bound used for pruning.

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
deficit. Its distance cannot decrease, so a branch whose lower bound is at least
the current upper bound cannot contain a better result. Pruning such a branch does
not change the minimum.

Compared with the source-aligned implementation, suppose an earlier sibling
improves the upper bound from `b_old` to `b_new`. A later sibling whose lower bound
is at least `b_new` cannot improve it, so rejecting that sibling immediately is
equivalent to entering it with `b_new` and rejecting its children there.

## Complexity

Let `M = 55` be the number of meld types, `P = 34` the number of pair types, `T = 34`
the number of tile types, and `k <= 4` the required number of melds. Without
pruning, the search has `binomial(M + d - 1, d)` nodes at depth `d` and tries all
`P` pairs at depth `k`. Since legality and distance checks scan `T` tile counts, the
worst-case time is

```text
O(T * (sum(d = 1..k, binomial(M + d - 1, d))
       + P * binomial(M + k - 1, k))).
```

All of these parameters are bounded by the fixed mahjong tile universe. In
practice, the four-copy check and distance-bound pruning remove most candidate
subtrees.

The updated-bound pruning does not change the worst-case complexity, but it can
avoid one level of unproductive work after an earlier sibling tightens the upper
bound.

The calculator stores `O(M)` meld data. Each calculation uses `O(T + k)` auxiliary
space for the target vector and recursion stack and performs no heap allocation.

## Implementation notes

The calculator constructor builds the 34 triplets followed by the 21 sequences
once. The resulting fixed-size table is reused for every calculation performed by
that calculator instance.

The implementation updates the target vector in place when entering and leaving a
branch. It recalculates the distance by scanning all 34 tile counts at each visited
candidate. A single mutable `upper_bound` is both updated by completed children and
read by the condition for subsequent siblings.

Keeping this version in a separate crate makes the optimization's benchmark effect
visible without changing the behavior of the source-aligned implementation.

## Correctness and limitations

- Exactness: theoretically exact because target enumeration is complete and pruning
  preserves the minimum; passes the shared exactness suite without a known-failure
  profile or ignored cases.
- Performance limitation: the search does not memoize repeated tile-count targets
  and rescans all 34 counts for each legality and distance check.

## Origin and references

- Base algorithm devised by: [Yoshitake Matsumoto (ymatsux)](https://github.com/ymatsux)
- Primary source: [`ShantensuUtil.java` from MjaiClients 0.1.5](https://github.com/gimite/MjaiClients/blob/0410a2f56c9e07621c50138d27006896eb9e4962/src/org/ymatsux/mjai/client/ShantensuUtil.java)
- Source-aligned Rust implementation: [`pruning_dfs_ymatsux`](../pruning_dfs_ymatsux)
- Additional reference: [枝刈りDFS - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/dfs/)

### Differences from the source

- Later siblings at the same depth are checked against the latest upper bound, not
  the bound supplied when the current call began.
- The number of melds is derived from the input tile count instead of always being
  four.
- The Java collections and utility classes are replaced by fixed-size Rust arrays.

## License

The original MjaiClients source declares the New BSD License and names Yoshitake Matsumoto as its author.
