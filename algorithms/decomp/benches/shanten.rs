#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp::Decomp;

shanten_benches!(Decomp);
