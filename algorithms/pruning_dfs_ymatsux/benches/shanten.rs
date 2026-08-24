#![feature(test)]

extern crate test;

use common::shanten_benches;
use pruning_dfs_ymatsux::PruningDfsYmatsux;

shanten_benches!(PruningDfsYmatsux);
