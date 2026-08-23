# xiangting Adapter

## Purpose

This crate adapts [`xiangting`](https://crates.io/crates/xiangting) 6.0.1 to the shared benchmark harness.

## Upstream and license

- Source: <https://github.com/Apricot-S/xiangting>
- Version: 6.0.1
- License: MIT

## Adapter configuration

- Four-player rules (`PlayerCount::Four`)
- Upstream input validation enabled by the public API
- Returns `replacement number - 1` as the conventional shanten number

## Calculation scope

The public upstream function returns the minimum over the general form, Seven Pairs, and Thirteen Orphans. It cannot be restricted to the general form. Its timings therefore do not represent the same calculation mode as adapters that calculate only the general form.

## Benchmark

```sh
cargo +nightly bench --package lib_xiangting
```
