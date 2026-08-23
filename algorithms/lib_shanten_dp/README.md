# shanten-dp Adapter

## Purpose

This crate adapts [`shanten-dp`](https://crates.io/crates/shanten-dp) 0.3.2 to the shared benchmark harness.

## Upstream and license

- Source: <https://github.com/tomohxx/shanten-dp-rust>
- Version: 0.3.2
- License: MIT

## Adapter configuration

- Four-player tile limits (`make_tile_limits(false)`)
- General form only (`Mode::STANDARD`)
- Four-identical-tile Seven Pairs disabled
- Argument validation disabled
- Required meld count calculated as `sum(hand) / 3`

## Calculation scope

Only the general-form shanten number is calculated. This differs from the `xiangting` adapter, which uses an upstream API that also considers Seven Pairs and Thirteen Orphans.

## Benchmark

```sh
cargo +nightly bench --package lib_shanten_dp
```
