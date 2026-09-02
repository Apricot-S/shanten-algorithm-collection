# Pruning DFS — mjai-manue-go Derivative

This algorithm searches complete winning-hand targets while accumulating only the
distance added by each selected block. It derives that increment from three tile
counts instead of rescanning the entire target and prunes melds that share no tile
with the input hand.

## Core idea

For a partial target, adding a meld changes at most three tile counts. The algorithm
maintains the target's current score and adds the marginal distance of each selected
meld. Triplet and sequence increments are computed directly from the affected tile
counts, making distance updates constant-time.

A candidate meld is discarded when all three of its tile occurrences are missing
from the input hand relative to the current target. Such a meld cannot belong to a
nearest target. A branch is also discarded when its accumulated score exceeds the
current upper bound.

## State and invariants

The depth-first search carries:

- `current`, the input tile-count vector;
- `target`, the partial target tile-count vector;
- `current_shanten`, the score of `target`;
- `melds_left`, the number of melds still to add;
- `min_meld_id`, the first meld allowed at the current node;
- `upper_bound`, the lowest shanten number found so far.

The meld table contains 34 triplets followed by 21 sequences. Triplet IDs are
strictly increasing because selecting the same triplet twice would require six
copies of one tile. Sequence IDs are nondecreasing, allowing repeated sequences
while avoiding permutations of the same meld multiset.

The target is updated in place and restored after every recursive call. A triplet
is rejected when its tile already occurs at least twice in the target, and a
sequence is rejected when adding it would make any tile count exceed four.

## Algorithm

The search starts with an empty target and score `-1`. After selecting the required
melds, it tries each possible pair and updates the upper bound.

```text
calculate(hand):
    target = empty tile-count vector
    melds_left = floor(sum(hand) / 3)
    return search(
        hand,
        target,
        current_shanten = -1,
        melds_left,
        first_meld = 0,
        upper_bound = 8,
    )

search(hand, target, current_shanten, melds_left, first_meld, upper_bound):
    if melds_left == 0:
        for each tile type:
            if a pair can be added without exceeding four copies:
                pair_distance = max(target[tile] + 2 - hand[tile], 0)
                upper_bound = min(
                    upper_bound,
                    current_shanten + pair_distance,
                )
        return upper_bound

    for each triplet with ID at least first_meld:
        skip if the same triplet cannot be added legally
        distance = marginal distance of adding the triplet
        new_shanten = current_shanten + distance
        if distance < 3 and new_shanten < upper_bound:
            add the triplet to target
            upper_bound = search(
                hand,
                target,
                new_shanten,
                melds_left - 1,
                first_meld = triplet.id + 1,
                upper_bound,
            )
            remove the triplet

    for each sequence with ID at least first_meld:
        skip if the sequence cannot be added legally
        distance = marginal distance of adding the sequence
        new_shanten = current_shanten + distance
        if distance < 3 and new_shanten < upper_bound:
            add the sequence to target
            upper_bound = search(
                hand,
                target,
                new_shanten,
                melds_left - 1,
                first_meld = sequence.id,
                upper_bound,
            )
            remove the sequence

    return upper_bound
```

Passing `triplet.id + 1` prevents the same triplet from being selected again.
Passing `sequence.id` permits a sequence to repeat. Because triplets precede
sequences in the table, no recursive call returns from sequence enumeration to
triplet enumeration.

### Shanten formula

For an input hand $h$ and any partial or complete target $g$, let $h_t$ and $g_t$ be
the counts of tile type $t$. Define

```math
\begin{aligned}
D(h, g) &= \sum_{t=0}^{33} \max(g_t - h_t, 0), \\
S(h, g) &= D(h, g) - 1
\end{aligned}
```

Only missing target tiles contribute to $D(h, g)$. For a complete target,
$S(h, g)$ is its shanten candidate. For a partial target, it is a lower bound for
every complete target descended from it.

For a meld $m$, the search updates the score using its marginal distance:

```math
\Delta(h, g, m) = D(h, g + m) - D(h, g)
```

For a sequence, this is one for each constituent tile whose count in the input does
not exceed its count in the current target:

```math
\Delta_{\mathrm{sequence}}(h, g, m)
= \sum_{t \in m} \mathbf{1}[h_t \leq g_t]
```

The triplet increment is evaluated directly from the single affected tile count.
At a leaf, the accumulated score is updated with the pair contribution inherited
from the upstream search:

```math
\begin{aligned}
P(h, g, t) &= \max(g_t + 2 - h_t, 0), \\
S_{\mathrm{candidate}}(h, g, t) &= S(h, g) + P(h, g, t)
\end{aligned}
```

When $g_t \leq h_t$, $P(h, g, t)$ is exactly the marginal increase in $D$. When
$g_t > h_t$, it overestimates that increase, whose actual value is two. Such a
candidate cannot uniquely improve the result: because a meld target occupies fewer
than 34 tile types, an unused tile type with $g_t = h_t = 0$ is available, and its
legal pair also has contribution two. The minimum pair contribution is therefore
preserved.

## Why it works

The canonical ID rules enumerate every legal meld multiset without exploring its
permutations: triplets occur at most once, while repeated sequences remain
reachable. Trying every legal pair after the required melds have been selected
completes each target.

The accumulated meld increments equal the change in missing-tile distance. Adding
another block cannot reduce $D(h, g)$ or $S(h, g)$, so a branch that reaches or
exceeds the upper bound cannot later improve it.

The zero-overlap rule discards a meld only when its marginal distance is three. It
uses the nearest-target lemma inherited from the Go implementation: an optimal
target can be decomposed and ordered so that every selected block contains at least
one input tile not already consumed by the partial target. A wholly missing meld
therefore is not required to attain the minimum. At the leaf, the pair calculation
also preserves the minimum as described above. The four-copy checks ensure that
every retained target remains physically legal.

## Complexity

Let $R = 34$ be the number of triplet types, $C = 21$ be the number of sequence
types, $P = 34$ be the number of pair types, and $k \leq 4$ be the number of required
melds. Ignoring legality and bound pruning, the number of canonical meld selections
at depth $d$ is

```math
N_d = \sum_{r=0}^{d}
      \binom{R}{r}
      \binom{C+d-r-1}{d-r}
```

Triplet, sequence, and pair distance updates inspect at most three tile counts. The
worst-case time is therefore

```math
O\left(\sum_{d=1}^{k} N_d + P N_k\right)
```

The compile-time meld table uses $O(R + C)$ constant data. Each calculation uses
$O(34 + k)$ auxiliary space for the target vector and recursion stack and performs
no heap allocation.

## Implementation notes

The 55 melds are generated by `const fn create_melds` and stored in the compile-time
constant `MELDS`. The calculator is a unit struct, so creating it performs no table
construction.

Unlike the ymatsux implementation in this workspace, the search never scans all 34
tile counts to update a partial target's distance. It mutates only the affected
counts and carries the accumulated score into the recursive call.

The upstream comparison uses `new_shanten <= upper_bound` because equal-score paths
can contribute additional `Goal` values. This derivative returns only the scalar
minimum, so it uses `new_shanten < upper_bound` and discards branches that can only
tie the current result.

## Correctness and limitations

- Exactness: theoretically exact because canonical enumeration is complete and the
  distance and zero-overlap pruning preserve the minimum; passes the shared
  exactness suite without a known-failure profile or ignored cases.
- Performance limitation: performs no memoization.

## Origin and references

- Derived from: [`mjai-manue-go` v0.3.0-beta.5, commit `1ead84275f75d1b4aafe68a6c6c6867e107379cb`](https://github.com/Apricot-S/mjai-manue-go/tree/1ead84275f75d1b4aafe68a6c6c6867e107379cb)
- Primary source: [`shanten.go` at the pinned commit](https://github.com/Apricot-S/mjai-manue-go/blob/1ead84275f75d1b4aafe68a6c6c6867e107379cb/internal/domain/game/round/service/shanten.go)
- Earlier implementation: [Gimite's `mjai-manue`](https://github.com/gimite/mjai-manue)

The lineage is Gimite's original `mjai-manue`, the corrected and optimized Go port,
and this benchmark-focused Rust derivative. It is not a faithful port of the
original Gimite implementation, which has known errors for hands containing four
identical tiles.

### Differences from the source

- Does not construct `Goal` values, block lists, required-tile vectors, or
  throwable-tile vectors.
- Omits the `AllowedExtraTiles` and `UpperBound` options and uses an initial upper
  bound of eight.
- Removes the slice growth, copying, filtering, and allocation associated with Goal
  enumeration from benchmark measurements.
- Uses a strict upper-bound comparison because equal-score targets do not need to
  be retained after Goal enumeration is removed.
- Replaces the Go package variables and block objects with a compile-time,
  fixed-size table of tile IDs.

## License

The original `mjai-manue-go` implementation is distributed under the BSD 3-Clause License and is Copyright 2024 Apricot S.
