# Algorithm Name

<!--
Copy this file to `algorithms/<crate>/README.md`, replace every placeholder, remove
instruction comments, and delete optional sections that do not apply.

Do not repeat workspace-wide assumptions such as the hand-form scope, input
representation, return-value convention, or validation policy. Document only facts
that distinguish this algorithm or its implementation.

Replace this introduction with a short summary of what the algorithm computes and
the idea that distinguishes it from the other implementations. A reader should be
able to decide whether to continue from this paragraph alone.
-->

## Core idea

<!--
Explain the mathematical or algorithmic observation that makes the method work.
Focus on why the approach finds a shanten number, rather than narrating the Rust
source line by line. Define algorithm-specific terms when they first appear.

For a derivative, explain the complete idea here and defer lineage and changes to
"Origin and differences" below.
-->

## State and invariants

<!--
Describe the information carried during the calculation: counts, partial
decompositions, tables, bounds, memoization keys, or other state. State the
invariants that are required for correctness, such as canonical enumeration order,
tile-count limits, or relationships between counters.

Delete this section if the algorithm has no meaningful state beyond its input.
-->

## Algorithm

<!--
Present the calculation in enough detail to reimplement it independently. Prefer
pseudocode for control flow and equations for scoring or bounds. Explain each
non-obvious pruning condition immediately after the pseudocode.
-->

```text
calculate(hand):
    initialize algorithm state
    enumerate or look up candidate structures
    return the best shanten number
```

### Shanten formula

<!--
Give the exact formula used to turn the algorithm state into a shanten number.
Define every variable and explain any algorithm-specific adjustments. Remove this
subsection if the value follows directly from the algorithm above.
-->

## Why it works

<!--
Give a concise correctness argument. Address both directions where applicable:

1. Completeness: why every relevant winning shape or decomposition is represented.
2. Soundness: why every represented candidate is legal and its score is valid.
3. Pruning: why a discarded branch cannot improve the current best result.

If the algorithm is intentionally inexact, replace the proof with the boundary of
the claim and a minimal counterexample.
-->

## Complexity

<!--
State worst-case time and auxiliary-space complexity using variables meaningful to
this algorithm. If a tight bound is not useful (for example, because the tile
universe is fixed), describe the dominant search space and the factors that reduce
it. Distinguish one-time table construction from per-hand work.
-->

## Implementation notes

<!--
Explain decisions that materially affect the implementation or benchmark result,
such as data layout, precomputation, allocation, cache reuse, search order, and
integer representation. Link to a small number of key source items when helpful.
Do not turn this section into an inventory of files or functions.
-->

## Correctness and limitations

<!--
State whether the implementation passes the shared exactness suite without ignored
cases. List only algorithm-specific limitations and known incorrect cases. For a
retained historical limitation, name the `shanten_tests!` profile and explain why
preserving the behavior is useful.
-->

- Exactness: <!-- exact, or intentionally limited -->
- Known incorrect cases: <!-- include a hand and expected/actual values when useful -->

## Origin and references

<!--
Credit the deviser and identify the source precisely. For a port or derivative,
include the upstream version or commit. List primary sources first, followed by
useful explanatory material.
-->

- Devised by: <!-- name and link -->
- Primary source: <!-- paper, article, repository, version, or commit -->
- Additional reference: <!-- reference -->

### Differences from the source

<!--
For a port or derivative, summarize behavior that was retained, removed, corrected,
or added, and explain whether this is a faithful port. Describe the effect of each
difference on behavior or performance. For an original algorithm, state that above
and delete this subsection.
-->

- <!-- difference -->

## License

<!--
State the license of incorporated or derived code and identify its copyright
holder. Link to the relevant entry in `THIRD-PARTY-NOTICES.md` when applicable.
For an original implementation, state that it uses the workspace license.
-->
