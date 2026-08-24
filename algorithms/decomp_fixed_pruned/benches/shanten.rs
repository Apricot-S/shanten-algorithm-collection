#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp_fixed_pruned::DecompFixedPruned;

shanten_benches!(DecompFixedPruned);
