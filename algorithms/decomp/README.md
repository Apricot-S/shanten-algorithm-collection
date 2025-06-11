# Block Decomposition

## Overview

1. Extract the pair.
2. Extract as many melds as possible.
3. Extract as many meld candidates as possible.
4. Calculate the shanten number:

    - Store the value if it is lower than other combinations.

5. Repeat for all possible combinations.

Constraint:

- **(number of melds) + (number of meld candidates) <= 4**.

Formula for shanten number:  
**8 - (number of melds) * 2 - (number of meld candidates) - (number of pairs (0 or 1))**

- If there are calls, subtract **2 * (number of calls)**.

## Features

- Use backtracking to find all melds-meld candidates combinations for the hand.
- Since isolated tiles are not explicitly identified, the algorithm does not correctly calculate the shanten number for hands lacking sufficient isolated tiles.
  - For more details on hands lacking sufficient isolated tiles, see [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## References

- [麻雀C言語プログラム集](https://web.archive.org/web/20190616213620/http://cmj3.web.fc2.com/)
