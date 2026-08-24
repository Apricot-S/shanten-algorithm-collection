# shanten-algorithm-collection

A Rust workspace for testing and benchmarking algorithms that calculate the shanten number (向聴数).

The shared correctness suite targets the general hand form: up to four melds and one pair. Seven Pairs (Chiitoitsu, 七対子) and Thirteen Orphans (Kokushi Musou, 国士無双) are outside that scope.

## Scope

The workspace provides:

- a shared `ShantenCalculator` interface
- correctness tests covering corner cases such as hands with insufficient isolated tiles
- four fixed-seed, 10,000-hand benchmark datasets

This project uses the definition of the shanten number presented in [結局のところ，麻雀における向聴数とは数学的かつ構成的にどう定義されるのか？ #数学 - Qiita](https://qiita.com/Cryolite/items/40908d0bce2bac310717).

## Correctness and known limitations

The exact implementations call `shanten_tests!(Type)` and must pass every shared case without `#[ignore]`.

The historical decomposition variants preserve their source algorithms. Their known failures are retained as reason-bearing ignored cases selected through named profiles. They cover these test ID groups:

- `test_shanten_waiting_for_the_5th_tile_*`
- selected `test_shanten_2_isolated_4_tiles_*` cases
- `test_shanten_4_honors_*`
- `test_shanten_lack_isolated_tile_*`

`decomp`, `decomp_pruned`, `decomp_ara`, and `decomp_ara_removal` also ignore `test_shanten_incomplete_hand_4_melds_without_a_pair`; `decomp_kobayashi` passes that case.

## Tests and benchmarks

Replace `your_algorithm` with the package you are adding or modifying.

```sh
cargo clippy --package your_algorithm --all-targets --all-features
cargo test --package your_algorithm
cargo bench --package your_algorithm --features benchmark
```

The benchmark groups are `normal`, `half_flush`, `full_flush`, and `thirteen_orphans`.

## Adding an implementation

See [algorithms/dummy](algorithms/dummy) for a minimal reference implementation.

1. Create a library crate under `algorithms/` and add it to the workspace members.
2. Inherit workspace package fields and lints in its `Cargo.toml`.
3. Expose the calculator type and implement `common::ShantenCalculator`; keep algorithm-specific helpers private.
4. Add `shanten_tests!(YourCalculator)` when the implementation claims exactness.
5. Add a feature-gated `benches/shanten.rs` target that invokes
   `shanten_benches!(YourCalculator)` to expose all four benchmark groups without
   running them during ordinary tests.
6. Document the origin, correctness, known constraints, and license in a crate README.
7. Record third-party code or dependencies in `THIRD-PARTY-NOTICES.md` as appropriate.

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT No Attribution license](LICENSE).
