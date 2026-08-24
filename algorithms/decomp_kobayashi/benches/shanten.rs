#![feature(test)]

extern crate test;

use common::shanten_benches;
use decomp_kobayashi::DecompKobayashi;

shanten_benches!(DecompKobayashi);
