# shanten-algorithm-collection

A Rust workspace for testing and benchmarking algorithms that calculate the riichi-mahjong shanten number.

Unless a row says otherwise, implementations calculate only the general hand form: up to four melds and one pair. Seven Pairs and Thirteen Orphans are outside that shared comparison scope.

## Scope

The workspace provides:

- a shared `ShantenCalculator` interface;
- 33 named correctness cases, including four-identical-tile and insufficient-isolated-tile regressions;
- four fixed-seed, 10,000-hand benchmark datasets; and
- adapters that expose selected third-party crates through the same benchmark harness.

See [this constructive definition of shanten](https://qiita.com/Cryolite/items/40908d0bce2bac310717) for the terminology used by the project.

## Algorithm comparison

| Crate | Origin | Method | Scope | Validation | Correctness status | External dependency |
| --- | --- | --- | --- | --- | --- | --- |
| [`decomp`](algorithms/decomp) | Mahjong C programs | Full block decomposition | General | No | Known four-tile and isolated-tile errors | No |
| [`decomp_pruned`](algorithms/decomp_pruned) | tomohxx | Pruned block decomposition | General | No | Same known errors as `decomp` | No |
| [`decomp_ara`](algorithms/decomp_ara) | Ara | Per-suit block decomposition | General | No | Known four-tile and isolated-tile errors | No |
| [`decomp_ara_removal`](algorithms/decomp_ara_removal) | Ara | Per-suit decomposition with isolated-tile removal | General | No | Known four-tile and isolated-tile errors | No |
| [`decomp_kobayashi`](algorithms/decomp_kobayashi) | Satoshi Kobayashi | Block-count decomposition | General | No | Known four-tile and isolated-tile errors | No |
| [`decomp_fixed_pruned`](algorithms/decomp_fixed_pruned) | tomohxx | Corrected, pruned decomposition | General | No | Passes all shared cases | No |
| [`pruning_dfs_ymatsux`](algorithms/pruning_dfs_ymatsux) | ymatsux | Target-hand pruning DFS | General | No | Passes all shared cases | No |
| [`pruning_dfs_mjai_manue_go`](algorithms/pruning_dfs_mjai_manue_go) | `mjai-manue-go` derivative | Incremental-distance pruning DFS | General | No | Passes all shared cases and the 40,000-hand differential check | No |
| [`lib_shanten_dp`](algorithms/lib_shanten_dp) | `shanten-dp` 0.3.2 | Dynamic programming adapter | General | Disabled | Upstream result; passes the adapter smoke test | Yes |
| [`lib_xiangting`](algorithms/lib_xiangting) | `xiangting` 6.0.1 | Upstream library adapter | Minimum of general, Seven Pairs, and Thirteen Orphans | Enabled | Upstream validated result | Yes |

The two external adapters do not calculate under identical conditions. In particular, `xiangting` includes special hand forms and input validation, while `shanten-dp` is configured for the general form with validation disabled. Their benchmark numbers must not be presented as a same-mode algorithm comparison.

### Pruning DFS variants

Both DFS implementations enumerate complete general-form targets and prune targets that cannot improve the current upper bound.

- `pruning_dfs_ymatsux` generates candidate melds during the search and evaluates target distance from the target state.
- `pruning_dfs_mjai_manue_go` reuses a compile-time meld table, updates distance incrementally, prevents duplicate triplet searches, and rejects melds sharing no tile with the input hand.
- The Go-derived crate calculates only shanten. It does not port upstream Goal enumeration or include its allocations in benchmarks.

The original Gimite implementation is intentionally not included because its handling of four identical tiles has known errors.

## Correctness and known limitations

The exact implementations call `shanten_tests!(Type)` and must pass every shared case without `#[ignore]`.

The historical decomposition variants preserve their source algorithms. Their known failures are retained as named, reason-bearing ignored cases. They cover these test ID groups:

- `test_shanten_waiting_for_the_5th_tile_*`;
- selected `test_shanten_2_isolated_4_tiles_*` cases;
- `test_shanten_4_honors_*`; and
- `test_shanten_lack_isolated_tile_*`.

`decomp`, `decomp_pruned`, `decomp_ara`, and `decomp_ara_removal` also ignore `test_shanten_incomplete_hand_4_melds_without_a_pair`; `decomp_kobayashi` passes that case. The `dummy` crate is not part of the correctness comparison and only tests that it always returns zero.

## External-crate comparison conditions

Every external adapter README uses this order: purpose, upstream and pinned version, license, adapter configuration, calculation scope, validation, result meaning, and commands. See [`lib_xiangting`](algorithms/lib_xiangting) and [`lib_shanten_dp`](algorithms/lib_shanten_dp).

Versions are exact requirements in their package manifests and resolved versions are recorded in `Cargo.lock`. License and derived-source notices are recorded in [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).

## Tests and benchmarks

The pinned nightly toolchain is selected automatically.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

Run the four shared benchmarks for one implementation with:

```sh
cargo bench --package pruning_dfs_ymatsux
```

The benchmark groups are `normal`, `half_flush`, `full_flush`, and `thirteen_orphans`. The last dataset is still evaluated as a general-form hand by general-only implementations.

Run the expensive cross-check for the Go-derived DFS explicitly:

```sh
cargo test --release --package pruning_dfs_mjai_manue_go \
  differential_tests::matches_exact_implementations_on_resources -- --ignored
```

## Adding an implementation

1. Create a library crate under `algorithms/` and add it to the workspace members.
2. Inherit workspace package fields and lints in its `Cargo.toml`.
3. Expose the calculator type and implement `common::ShantenCalculator`; keep algorithm-specific helpers private.
4. Add `shanten_tests!(YourCalculator)` when the implementation claims exactness.
5. Add `shanten_benches!(YourCalculator)` to expose all four benchmark groups.
6. Document the origin, pseudocode, pruning rules, correctness, constraints, and license in an English crate README.
7. Record third-party code or dependencies in `THIRD-PARTY-NOTICES.md` and `Cargo.lock` as appropriate.

Algorithm ports should retain the source's search order, pruning behavior, and state representation unless a documented bug fix requires a change.

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT No Attribution License](LICENSE). Third-party components remain under the licenses listed in [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).
