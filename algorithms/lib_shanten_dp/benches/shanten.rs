#![feature(test)]

extern crate test;

use common::shanten_benches;
use lib_shanten_dp::ShantenDp;

shanten_benches!(ShantenDp);
