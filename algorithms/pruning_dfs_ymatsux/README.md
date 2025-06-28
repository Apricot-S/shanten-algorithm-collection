# Pruning DFS - ymatsux

## Devised by

Yoshitake Matsumoto (ymatsux)

## Overview

1. 全ての面子（順子・刻子）の組み合わせを再帰的に探索する。
2. 残り面子数分だけ探索を進める。
3. 探索の途中で、現在の手牌から作れる最小の向聴数（下界）を計算し、既に見つかっている最小値（上界）より悪い場合はそれ以上探索しない（枝刈りを行う）。
4. 面子が揃ったら雀頭の位置を全て試す。
5. 向聴数を計算し、最小値を記録する。

### Formula for shanten number

`target`（探索中の和了形）と`hand`（元の手牌）の差分から以下の式で計算：

**向聴数 = targetとhandの差分合計 - 1**

（targetの各牌枚数からhandの各牌枚数を引き、0未満は0として合計し、最後に1を引く）

### Features

- 与えられた手牌の全ての和了形との差分の最小値を計算する。
- 向聴の定義通りの値を計算するため、正確な向聴数を計算できることが理論的に保証されている。
- 全ての面子・雀頭の組み合わせをDFSで探索する。
- 枝刈りによって不要な探索を省略する。
- 探索中に現在の手牌から作れる最小の向聴数（下界）を計算し、既に見つかっている最小値（上界）より悪い場合はそれ以上探索しない。

## References

- <https://github.com/gimite/MjaiClients/blob/master/src/org/ymatsux/mjai/client/ShantensuUtil.java>
- [枝刈りDFS - 麻雀アルゴリズム](https://tomohxx.github.io/mahjong-algorithm-book/dfs/)
