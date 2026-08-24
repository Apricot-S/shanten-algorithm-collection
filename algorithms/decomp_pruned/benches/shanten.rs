#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp_pruned::DecompPruned;

shanten_benches!(DecompPruned);
