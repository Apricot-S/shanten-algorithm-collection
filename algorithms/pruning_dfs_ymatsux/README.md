# Pruning DFS — ymatsux

This algorithm calculates the general-form shanten number by searching complete
winning-hand targets. It avoids most of the exhaustive search by pruning a partial
target as soon as its distance from the input hand cannot improve the best result
found so far.

## Core idea

A general-form winning hand consists of a fixed number of melds and one pair. The
algorithm constructs every such target from the empty hand, then measures how many
tiles would have to be added to the input hand to reach it.

Adding a meld or pair to a partial target can never reduce this distance. The
distance of a partial target is therefore a lower bound for all of its descendants.
Once that bound reaches the best complete-target distance already found, the entire
branch can be discarded.

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
            upper_bound = search(
                target,
                melds_left - 1,
                first_meld = meld.id,
                upper_bound,
            )
        remove the meld

    return upper_bound
```

### Shanten formula

For an input hand `h` and a complete target `g`, the shanten number associated with
that target is

```text
D(h, g) - 1 = sum(max(g[tile] - h[tile], 0)) - 1.
```

Only missing target tiles contribute to `D`: tiles in the input hand that are not
used by the target can be discarded during the corresponding tile exchanges. The
minimum value over all legal targets is the general-form shanten number.

The same expression applied to a partial target is the lower bound used for
pruning.

## Why it works

Every legal meld is present in the 55-entry table, every legal pair is tried at the
leaf, and nondecreasing meld IDs enumerate every meld multiset. Consequently, every
legal general-form winning target with the required number of melds is represented
by at least one visited leaf.

Conversely, a leaf consists only of legal melds and one pair, and the four-copy
check rejects targets that cannot be a physical mahjong hand. Its distance is
therefore the number of additions required to reach a legal target, minus one by
the definition of shanten.

Extending a partial target can only preserve or increase each positive tile-count
deficit. Its distance cannot decrease, so a branch whose lower bound is at least
the current upper bound cannot contain a better result. Pruning such a branch does
not change the minimum.

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

The calculator stores `O(M)` meld data. Each calculation uses `O(T + k)` auxiliary
space for the target vector and recursion stack and performs no heap allocation.

## Implementation notes

`PruningDfsYmatsux::new` constructs the 34 triplets followed by the 21 sequences
once. The resulting fixed-size table is reused for every calculation performed by
that calculator instance.

The implementation updates the target vector in place when entering and leaving a
branch. It recalculates the distance by scanning all 34 tile counts at each visited
candidate. The upper bound is tightened immediately after any better leaf and is
then reused while examining the remaining siblings.

## Correctness and limitations

- Exactness: invokes the shared exactness suite without a known-failure profile or
  ignored cases.
- Known incorrect cases: none within the workspace's general-form scope.
- Performance limitation: the search does not memoize repeated tile-count targets
  and rescans all 34 counts for each legality and distance check.

## Origin and references

- Devised by: [Yoshitake Matsumoto (ymatsux)](https://github.com/ymatsux)
- Primary source: [`ShantensuUtil.java` from MjaiClients 0.1.5](https://github.com/gimite/MjaiClients/blob/0410a2f56c9e07621c50138d27006896eb9e4962/src/org/ymatsux/mjai/client/ShantensuUtil.java)
- Additional reference: [枝刈りDFS - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/dfs/)

### Differences from the source

- This crate calculates only the general form. The Java source takes the minimum
  over the general form and Seven Pairs.
- The number of melds is derived from the input tile count instead of always being
  four, allowing the shared representation to account for open melds and shorter
  hands.
- The current best result is used to prune later siblings at the same depth. The
  source compares those siblings with the bound supplied on entry to that call.
- The Java collections and utility classes are replaced by fixed-size Rust arrays;
  the target-search method itself is preserved.

## License

The original MjaiClients source declares the New BSD License and names Yoshitake
Matsumoto as its author.
