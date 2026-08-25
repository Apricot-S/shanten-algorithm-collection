# xiangting Adapter

## Purpose

This crate adapts `xiangting` to the shared benchmark harness.

## Upstream and pinned version

- Source: <https://github.com/Apricot-S/xiangting>
- Version: exactly 6.0.1

## License

MIT. See the workspace `THIRD-PARTY-NOTICES.md`.

## Adapter configuration

- Four-player rules (`PlayerCount::Four`)
- The upstream public replacement-number API

## Calculation scope

The result is the minimum over the general form, Seven Pairs, and Thirteen Orphans. The public upstream function cannot be restricted to the general form.

## Validation

Upstream input validation is enabled by the public API. Benchmark inputs are expected to be valid four-player hands.

## Result meaning

The upstream replacement number is converted to the conventional shanten number by subtracting one. Because special hand forms and validation are included, timings are not a same-mode comparison with the general-only `shanten-dp` adapter.

## Commands

```sh
cargo test --package sac_xiangting
cargo bench --package sac_xiangting --features benchmark
```
