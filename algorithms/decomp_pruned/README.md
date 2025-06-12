# Block Decomposition - Pruned

## Devised by

[tomohxx](https://github.com/tomohxx)

> [!NOTE]
> This implementation applies only part of tomohxx's original algorithm (specifically, the pruning optimization). The correction for miscalculations is not included, in order to isolate the effect of pruning.
> The algorithm with all proposals applied is available in this repository as [decomp_pruned_fixed](../decomp_pruned_fixed).

## Overview

(TODO)

Constraint:

- **(number of melds) + (number of meld candidates) <= 4**

Formula for shanten number:  
**8 - (number of melds) * 2 - (number of meld candidates) - (number of pairs (0 or 1))**

- If there are calls, subtract **(number of calls) * 2**.

## Features

- An improved version of the algorithm originally devised by [麻雀C言語プログラム集](https://web.archive.org/web/20190616213620/http://cmj3.web.fc2.com/).
- Since isolated tiles are not explicitly identified, the algorithm does not correctly calculate the shanten number for hands lacking sufficient isolated tiles.
  - For more details on hands lacking sufficient isolated tiles, see [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## References

- [tomohxx/shanten-test](https://github.com/tomohxx/shanten-test)
- [正確なブロック分解方式の向聴数計算アルゴリズムの提案](https://zenn.dev/tomohxx/articles/16c0d807218d2a)
