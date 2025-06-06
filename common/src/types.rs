use crate::constants::TILE_TYPE_COUNT;

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
pub type TileCounts = [TileCount; TILE_TYPE_COUNT as usize];
