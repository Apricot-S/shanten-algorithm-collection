#![feature(test)]

extern crate test;

use common::shanten_benches;
use pruning_dfs_mjai_manue_go::PruningDfsMjaiManueGo;

shanten_benches!(PruningDfsMjaiManueGo);
