# common

This crate contains the reusable API and harness implementation used by the algorithm crates.
It does not calculate shanten numbers itself.

## Public API

- `ShantenCalculator` is the interface implemented by every calculator.
- `TileCount` is the count for one tile type (`u8`).
- `TileCounts` is the 34-element array passed to calculators.
- `TileCountsExt::from_code` converts a Tenhou-style hand string into `TileCounts`.
- `shanten_tests!` generates the shared correctness tests.
- `shanten_benches!` generates benchmarks for the four bundled datasets.
- `NUM_TILE_TYPES`, `MAX_NUM_TILE`, `MIN_HAND_SIZE`, and `MAX_HAND_SIZE` describe
  the common representation limits.

`TileCounts` uses the standard 34-tile ordering:

| Indices   | Tiles                                                           |
| --------- | --------------------------------------------------------------- |
| `0..=8`   | `1m` through `9m`                                               |
| `9..=17`  | `1p` through `9p`                                               |
| `18..=26` | `1s` through `9s`                                               |
| `27..=33` | `1z` through `7z` (east, south, west, north, white, green, red) |

## Implementing a calculator

Implement `ShantenCalculator` and invoke the test macro at the crate root. A return
value of `0` means tenpai, and `-1` means a complete hand.

```rust
use common::{ShantenCalculator, TileCounts};

pub struct MyCalculator;

impl ShantenCalculator for MyCalculator {
    fn new() -> Self {
        Self
    }

    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        // Replace this with the algorithm implementation.
        todo!("calculate the shanten number for {hand:?}")
    }
}

common::shanten_tests!(MyCalculator);
```

An exact implementation should use the single-argument form shown above so that all
shared cases must pass.

Historical decomposition algorithms with documented limitations can select a
known-failure profile instead:

```rust
common::shanten_tests!(
    MyCalculator,
    profile = legacy_decomposition,
    reason = "preserves the known limitations of the original algorithm",
);
```

`legacy_decomposition` ignores all known historical decomposition failures.
`legacy_decomposition_with_incomplete_hand_support` requires the incomplete-hand case
to pass while ignoring the other known failures. These profiles are intended only for
faithful ports whose limitations are explained in their crate documentation.

For tests and utilities, a hand can be written in Tenhou notation:

```rust
use common::{TileCounts, TileCountsExt};

let hand = TileCounts::from_code("123m456p789s11222z");
assert_eq!(hand.iter().sum::<u8>(), 14);
```

## Benchmarks

The benchmark macro generates one benchmark for each fixed-seed, 10,000-hand dataset:

- normal hands
- half-flush hands
- full-flush hands
- Thirteen Orphans-oriented hands

It uses Rust's unstable `test` crate. Keep the benchmark in a feature-gated benchmark
target so that neither `cargo test` nor `cargo test --all-targets` executes it:

```toml
[features]
benchmark = []

[lib]
bench = false

[[bench]]
name = "shanten"
path = "benches/shanten.rs"
required-features = ["benchmark"]
```

Place the harness in `benches/shanten.rs`:

```rust
#![feature(test)]

extern crate test;

use my_algorithm::MyCalculator;

common::shanten_benches!(MyCalculator);
```

The generated harness reads the datasets from the workspace's `resources` directory.
Calculator crates can run their benchmarks with:

```sh
cargo +nightly bench --package my_algorithm --features benchmark
```

See [`algorithms/dummy`](../algorithms/dummy) for the smallest integration example and
the [workspace README](../README.md) for contribution and correctness requirements.
