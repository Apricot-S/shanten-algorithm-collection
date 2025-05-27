use common::shanten_tests;
use common::{ShantenCalculator, TileCounts};

struct Dummy {}

impl ShantenCalculator for Dummy {
    fn new() -> Self {
        Dummy {} // ダミー実装のため、特に初期化は行わない
    }

    /// シャンテン数を計算するダミー実装
    fn calculate_shanten(&self, _hand: &TileCounts) -> i8 {
        0 // ダミー実装
    }
}

// マクロを使用してテストケースを定義
shanten_tests!(Dummy);
