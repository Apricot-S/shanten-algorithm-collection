# shanten-dp Adapter

## Purpose

This crate adapts `shanten-dp` to the shared benchmark harness.

## Upstream and pinned version

- Source: <https://github.com/tomohxx/shanten-dp-rust>
- Version: exactly 0.3.2

## License

MIT. See the workspace `THIRD-PARTY-NOTICES.md`.

## Adapter configuration

- Four-player tile limits: `make_tile_limits(false)`
- General form only: `Mode::STANDARD`
- Four-identical-tile Seven Pairs disabled: `four_tile_seven_pairs = false`
- Required meld count: `m = sum(hand) / 3`

## Calculation scope

Only the general-form shanten number is calculated. This differs from the `xiangting` adapter, which also considers Seven Pairs and Thirteen Orphans.

## Validation

Argument validation is disabled with `check_hand = false`.

## Result meaning

The adapter returns the upstream general-form shanten number directly and runs the shared correctness suite.

## Commands

```sh
cargo test --package lib_shanten_dp
cargo bench --package lib_shanten_dp
```
