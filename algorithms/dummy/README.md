# Dummy

## Purpose

This crate is the minimal reference implementation for integrating an algorithm with
the shared interface and benchmark harness.

## Behavior

`Dummy` implements `common::ShantenCalculator` but always returns `0`. It is only a
structural example and does not calculate the shanten number correctly.

The source includes a commented `common::shanten_tests!(Dummy)` invocation to show
where an actual implementation enables the shared correctness suite. It remains
disabled because the dummy result does not satisfy those tests.

The benchmark target invokes `common::shanten_benches!(Dummy)` and exposes all four
shared benchmark groups when the `benchmark` feature is enabled.

## Commands

```sh
cargo test --package dummy
cargo bench --package dummy --features benchmark
```
