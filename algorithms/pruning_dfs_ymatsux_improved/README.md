# Pruning DFS — ymatsux with Updated-Bound Pruning

This algorithm is a small improvement to the ymatsux pruning DFS. It searches the
same complete winning-hand targets but applies every newly improved upper bound to
the remaining sibling branches immediately, avoiding recursive calls that the
original control flow would enter before rejecting their descendants.

## Core idea

The base algorithm constructs complete winning-hand targets. For a partial target,
the number of tiles missing from the input hand minus one is a lower bound: adding
more blocks cannot reduce an existing tile deficit. A branch can therefore be
discarded when its lower bound cannot improve the best complete target already
found.

The original implementation preserves the upper bound supplied on entry to each
search call when checking all siblings in that call. This version instead mutates
that bound whenever a child returns a better result. Every later sibling is checked
against the latest value.

## State and invariants

The search carries a 34-element partial target, the number of melds still required,
the minimum allowed meld ID, and a mutable `upper_bound`.

The meld table contains 34 triplets and 21 sequences. Nondecreasing meld IDs visit
each meld multiset once. The target is updated in place and restored after every
branch, no tile count may exceed four, and extending a target cannot decrease its
distance from the input hand.

## Algorithm

The required number of melds is `floor(tile_count / 3)`. At the leaf, all 34 pair
types are tried.

```text
search(target, melds_left, first_meld, upper_bound):
    if melds_left == 0:
        for each legal pair:
            upper_bound = min(upper_bound, distance(hand, target + pair) - 1)
        return upper_bound

    for each meld with ID at least first_meld:
        add the meld to target
        lower_bound = distance(hand, target) - 1
        if target is legal and lower_bound < upper_bound:
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

For an input hand `h` and target `g`, the distance-based shanten value is

```text
sum(max(g[tile] - h[tile], 0)) - 1.
```

The minimum over all legal complete targets is returned.

## Why it works

The target enumeration and distance calculation are unchanged from the base
algorithm. Every legal target is represented, and every visited leaf is made from
legal melds and one pair subject to the four-copy limit.

Suppose an earlier sibling improves the upper bound from `b_old` to `b_new`. Any
later sibling whose lower bound is at least `b_new` can only preserve or increase
that distance in its descendants. It cannot improve `b_new`, so rejecting it
immediately produces the same minimum as entering it with `b_new` and rejecting its
children there.

## Complexity

Let `M = 55` be the number of meld types, `P = 34` the number of pairs, `T = 34` the
number of tile types, and `k <= 4` the number of required melds. The unpruned
worst-case time remains

```text
O(T * (sum(d = 1..k, binomial(M + d - 1, d))
       + P * binomial(M + k - 1, k))).
```

The improvement does not change the worst-case bound, but it can avoid one level of
otherwise unproductive work after an earlier sibling tightens the upper bound. The
calculator stores `O(M)` meld data, and each calculation uses `O(T + k)` auxiliary
space without heap allocation.

## Implementation notes

The implementation is intentionally identical to `pruning_dfs_ymatsux` except for
the lifetime of the pruning bound. A single mutable `upper_bound` is both updated by
completed children and read by the condition for subsequent siblings.

Keeping this version in a separate crate makes the optimization's benchmark effect
visible without changing the behavior of the source-aligned implementation.

## Correctness and limitations

- Exactness: invokes the shared exactness suite without a known-failure profile or
  ignored cases.
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
