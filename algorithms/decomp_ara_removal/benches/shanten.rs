#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp_ara_removal::DecompAraRemoval;

shanten_benches!(DecompAraRemoval);
