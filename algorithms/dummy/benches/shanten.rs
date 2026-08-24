#![feature(test)]

extern crate test;

use common::shanten_benches;
use dummy::Dummy;

shanten_benches!(Dummy);
