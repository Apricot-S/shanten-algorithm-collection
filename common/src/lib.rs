//! シャンテン数計算アルゴリズムで使用する定数や型定義

mod test;

// 同一の牌の最大枚数
pub const MAX_TILE_COUNT: i8 = 4;
// 牌の種類の数
pub const TILE_TYPE_COUNT: i8 = 34;
// 手牌の最小枚数
pub const MIN_HAND_SIZE: i8 = 1;
// 手牌の最大枚数
pub const MAX_HAND_SIZE: i8 = 14;

/// 牌の枚数を表す数
pub type TileCount = u8;

/// 34種類の牌の枚数を表す配列
///
/// 牌の種類は、0から33までの整数で表される
/// インデックスと牌の種類は以下のように対応する
/// 0: 1m, 1: 2m, ..., 8: 9m,
/// 9: 1p, ..., 17: 9p,
/// 18: 1s, ..., 26: 9s,
/// 27: 東, 28: 南, 29: 西, 30: 北, 31: 白, 32: 發, 33: 中
pub type TileCounts = [u8; TILE_TYPE_COUNT as usize];

pub trait ShantenCalculator {
    fn new() -> Self;
    fn calculate_shanten(&self, hand: &TileCounts) -> i8;
}
