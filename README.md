# shanten-algorithm-collection

A Rust workspace for testing and benchmarking algorithms that calculate the riichi-mahjong shanten number.

The shared correctness suite targets the general hand form: up to four melds and one pair. Seven Pairs and Thirteen Orphans are outside that scope.

## Scope

The workspace provides:

- a shared `ShantenCalculator` interface;
- 33 named correctness cases, including four-identical-tile and insufficient-isolated-tile regressions;
- four fixed-seed, 10,000-hand benchmark datasets; and
- adapters that expose selected third-party crates through the same benchmark harness.

See [this constructive definition of shanten](https://qiita.com/Cryolite/items/40908d0bce2bac310717) for the terminology used by the project.

## Correctness and known limitations

The exact implementations call `shanten_tests!(Type)` and must pass every shared case without `#[ignore]`.

The historical decomposition variants preserve their source algorithms. Their known failures are retained as named, reason-bearing ignored cases. They cover these test ID groups:

- `test_shanten_waiting_for_the_5th_tile_*`;
- selected `test_shanten_2_isolated_4_tiles_*` cases;
- `test_shanten_4_honors_*`; and
- `test_shanten_lack_isolated_tile_*`.

`decomp`, `decomp_pruned`, `decomp_ara`, and `decomp_ara_removal` also ignore `test_shanten_incomplete_hand_4_melds_without_a_pair`; `decomp_kobayashi` passes that case. The `dummy` crate is not part of the correctness comparison and only tests that it always returns zero.

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

Licensed under the [MIT No Attribution license](LICENSE).
