# Pruning DFS — mjai-manue-go Derivative

## Origin

This implementation is a shanten-only Rust derivative of the general-form DFS in [`mjai-manue-go` v0.3.0-beta.5](https://github.com/Apricot-S/mjai-manue-go/tree/v0.3.0-beta.5), commit `1ead84275f75d1b4aafe68a6c6c6867e107379cb`. The lineage is Gimite's `mjai-manue`, the corrected and optimized Go port, and then this benchmark-focused Rust derivative.

It is not a faithful port of the original Gimite implementation, which has known errors for hands containing four identical tiles.

## Algorithm

The DFS constructs a general-form winning target in canonical order. It tracks the distance accumulated by each selected meld instead of rescanning all 34 tile counts at every node.

```text
search(target, current_distance, melds_left, first_meld, upper_bound):
    if melds_left == 0:
        try every legal pair and update upper_bound

    try each remaining triplet that shares at least one tile with the hand
    try each remaining sequence that shares at least one tile with the hand
    recurse only when the accumulated lower bound is within upper_bound
```

The meld table is constructed at compile time and reused by every calculation. Triplets are followed by sequences, and IDs are nondecreasing within each group, which avoids duplicate target enumeration.

## Differences from upstream

- Returns only the general-form shanten number.
- Does not construct `Goal` values or winning-hand block lists.
- Does not calculate required or throwable tile vectors.
- Does not expose `AllowedExtraTiles` or `UpperBound` options.
- Excludes all allocation costs associated with Goal enumeration from benchmarks.

## Correctness

The shared exactness cases cover four-identical-tile, fifth-tile-wait, and insufficient-isolated-tile regressions. An additional ignored differential test compares all 40,000 resource hands with the ymatsux pruning DFS and the corrected decomposition implementation.

Run it explicitly with:

```sh
cargo +nightly test --release --package pruning_dfs_mjai_manue_go \
  differential_tests::matches_exact_implementations_on_resources -- --ignored
```

## License

The upstream Go implementation is distributed under the BSD 3-Clause License and is Copyright 2024 Apricot S.
