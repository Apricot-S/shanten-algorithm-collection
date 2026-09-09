# Block Decomposition — tomohxx

This algorithm corrects the block-decomposition method for hands with insufficient
isolated tiles and adds lower-bound pruning to the meld-candidate search. It counts
isolated tiles only for the few terminal block patterns where the ordinary shanten
formula can underestimate the result.

## Core idea

The algorithm starts from the same exhaustive decomposition as [`decomp`](../decomp):
it reserves each possible head, extracts complete melds, and then extracts two-tile
meld candidates. It makes three changes to that method.

First, after reserving a head, the remaining two copies of the same tile type cannot
also be extracted as a pair-shaped meld candidate. This prevents a four-copy group
from being credited as both the head and a pair wait.

Second, the ordinary block formula assumes that enough remaining isolated tiles can
supply the missing head or meld. That assumption can fail only for three terminal
block-count patterns. The algorithm checks isolated-tile availability for those
patterns and adds one to the candidate score when the required tiles are absent.
Other patterns use the ordinary formula without scanning the remaining tiles.

Third, once meld extraction finishes, the selected meld and head counts determine
the best score that any continuation of the meld-candidate search could attain. If
the best score already found is no greater than this lower bound, the entire
candidate search for that meld decomposition is skipped.

## State and invariants

The search carries:

- `hand`, the tile counts remaining after the current extractions;
- `original`, the unchanged input counts used by the isolated-tile checks;
- `BlockCounts { melds, meld_candidates, pairs }`, where `melds` includes called
  melds and `pairs` is zero or one;
- `pair_index`, the tile type reserved as the head, or a sentinel when no head was
  reserved;
- `min_shanten`, the lowest score found so far;
- `lower_bound`, the minimum score attainable from the current meld decomposition.

Every extraction consumes available tiles and is undone before its call returns.
Starting indices are nondecreasing within each extraction phase, so repeated blocks
remain possible while permutations of the same extraction order are avoided.
Meld-candidate extraction maintains
`melds + meld_candidates <= 4`.

The original counts never change. A pair-shaped meld candidate must have a different
index from `pair_index`; this invariant is required by the four-copy correction.
The lower bound is calculated once when the search moves from meld extraction to
meld-candidate extraction and remains valid throughout that candidate subtree.

## Algorithm

Let `called_melds = 4 - floor(sum(hand) / 3)`. The search initializes `melds` with
this value, tries each possible head, and also searches without a head.

```text
calculate(input):
    hand = copy of input
    original = input
    counts = {
        melds = called_melds,
        meld_candidates = 0,
        pairs = 0,
    }
    best = 8

    for each tile type t with hand[t] >= 2:
        remove (t, t); counts.pairs = 1
        cut_meld(pair_index = t, i = 0)
        restore (t, t); counts.pairs = 0

    cut_meld(pair_index = none, i = 0)
    return best

cut_meld(pair_index, i):
    if i == 34:
        lower_bound = 4 - counts.melds - counts.pairs
        cut_candidate(pair_index, i = 0, lower_bound)
        return

    for each available meld starting at i, in this order:
        triplet (i, i, i)
        sequence (i, i+1, i+2)
        remove meld; counts.melds += 1
        cut_meld(pair_index, i)
        restore meld; counts.melds -= 1

    cut_meld(pair_index, i + 1)

cut_candidate(pair_index, i, lower_bound):
    if best <= lower_bound:
        return

    if i == 34:
        evaluate_terminal_decomposition()
        return

    if counts.melds + counts.meld_candidates < 4:
        for each available candidate starting at i, in this order:
            pair (i, i), only when hand[i] == 2 and i != pair_index
            adjacent-tile candidate (i, i+1)
            gapped-tile candidate (i, i+2)
            remove candidate; counts.meld_candidates += 1
            cut_candidate(pair_index, i, lower_bound)
            restore candidate; counts.meld_candidates -= 1

    cut_candidate(pair_index, i + 1, lower_bound)
```

At a terminal decomposition, let $(m,t,p)$ denote `(melds, meld_candidates,
pairs)`. Isolated tiles are examined only in these cases:

| Block counts $(m,t,p)$ | Required remaining tiles                                    |
| ---------------------- | ----------------------------------------------------------- |
| $(4,0,0)$ or $(3,1,0)$ | One tile eligible to become the head                        |
| $(3,0,1)$              | One tile eligible to become a meld                          |
| $(3,0,0)$              | Two tile types eligible to supply the missing head and meld |

For the first case, the implementation accepts a remaining tile only when its
original count is less than three. For the other two cases, it accepts any remaining
suited tile, or a remaining honor whose original count is less than three. The last
case counts eligible tile types and requires at least two. If the applicable
requirement is not met, the terminal score is increased by one.

### Shanten formula

For a decomposition with $m$ melds including calls, $t$ meld candidates, and
$p \in \{0,1\}$ reserved heads, the ordinary candidate score is

```math
S = 8 - 2m - t - p,
\qquad m+t \leq 4
```

For one of the three isolated-tile patterns, let $q$ be one when the required
isolated tiles are unavailable and zero otherwise. Its corrected score is

```math
S_{\mathrm{corrected}} = 8 - 2m - t - p + q
```

When meld extraction ends, at most $4-m$ candidates can be added. Substituting that
maximum into the ordinary formula gives the smallest score reachable by the
candidate subtree:

```math
L = 8 - 2m - (4-m) - p = 4 - m - p
```

Thus, if `min_shanten <= L`, no continuation can lower `min_shanten` and the subtree
can be pruned. The isolated-tile correction can only add one, so it cannot invalidate
this lower bound.

## Why it works

The head loop, meld search, and candidate search retain the exhaustive block choices
of the base decomposition. Nondecreasing indices remove only extraction-order
duplicates. Excluding `pair_index` from pair-shaped candidates rejects a score that
would require the same four-copy group to serve as both the head and an additional
pair wait.

The isolated-tile analysis cited below classifies every insufficient-isolated-tile
hand into the three terminal patterns checked by this implementation. For those
patterns, the base formula is one too low exactly when the required eligible tiles
are absent, so adding one corrects the result. No correction is needed outside
those patterns. The pruning rule follows directly from $L$: every descendant has a
score at least $L$, including any isolated-tile adjustment.

The source does not provide a theoretical proof of the complete algorithm. Its
claim is supported instead by exhaustive comparison of every 13-tile and 14-tile
hand against an algorithm with a correctness proof.

## Complexity

Let $T=34$ be the number of tile types and $n$ the number of input tiles. There are
at most $T+1$ head choices. For each choice, the algorithm enumerates competing meld
decompositions and, unless pruned, their meld-candidate decompositions. The search
space is combinatorial; the lower bound reduces practical work but does not improve
the loose exponential worst-case time bound in $T+n$.

An extraction recurses at the same index but removes at least two tiles. Advance
calls traverse at most $T$ indices in each of the two phases. A recursion path has
depth $O(T+n)$. Each isolated-tile correction scans $T$ counts, but only the three
listed terminal patterns trigger that scan.

Auxiliary space is $O(T+n)$ for the copied hand and recursion stack. The algorithm
uses no lookup table, cache, or per-branch heap allocation.

## Implementation notes

The implementation updates one hand copy in place and backtracks after each branch.
It retains the original counts separately because eligibility of an isolated tile
depends on both its remaining and original counts. `BlockCounts` and shanten values
use `i8`.

[`cut_meld_cand`](src/lib.rs) checks the lower bound before inspecting the current
index, so a pruned subtree performs no further candidate enumeration. The isolated-
tile helpers scan the fixed-size count arrays directly and allocate no collections.

## Correctness and limitations

- Exactness: no theoretical proof is given in the primary source. The upstream
  implementation was exhaustively validated for every 13-tile and 14-tile hand,
  and this implementation passes the shared exactness suite without ignored cases.
- Known incorrect cases: none within the supported input scope.

## Origin and references

- Devised by: [tomohxx](https://github.com/tomohxx)
- Primary source:
  [正確なブロック分解方式の向聴数計算アルゴリズムの提案](https://zenn.dev/tomohxx/articles/16c0d807218d2a)
- Reference implementation:
  [`shanten.cpp` from `tomohxx/shanten-test` at commit `8dd8d41`](https://github.com/tomohxx/shanten-test/blob/8dd8d41f1179997a5ce7f979a616b551ec40c868/src/shanten.cpp)
- Additional reference:
  [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1)

### Differences from the source

- This implementation infers the number of called melds from the input tile count.
  The C++ interface receives that number as an explicit argument.
- The remaining search, correction, and pruning behavior is preserved in the Rust port.

## License

The upstream `tomohxx/shanten-test` implementation is distributed under the
[MIT License](https://github.com/tomohxx/shanten-test/blob/8dd8d41f1179997a5ce7f979a616b551ec40c868/LICENSE).
