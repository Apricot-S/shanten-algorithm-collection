#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp_ara::DecompAra;

shanten_benches!(DecompAra);
