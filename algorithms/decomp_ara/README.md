# Block Decomposition - Ara

## Devised by

[Ara](https://mahjong.ara.black/intro/selfintro.htm)

## Overview

1. Extract a pair (also consider the pattern where no pair is extracted).
2. Extract melds and meld candidates from honors.
3. Extract melds and meld candidates from each suit:

    - Search for combinations of A and B as follows:

      - A: The combination that maximizes **(number of melds) * 2 + (number of meld candidates)**.
      - B: The combination that maximizes **(number of melds) * 10 + (number of meld candidates)**.

4. Calculate the shanten number for all the combinations of A and B.

    - Store the value if it is lower than other combinations.

5. Repeat for all possible combinations.

### Constraint

- **(number of melds) + (number of meld candidates) <= 4**

### Formula for shanten number

**8 - (number of melds) * 2 - (number of meld candidates) - (number of pairs (0 or 1))**

If there are calls, subtract **(number of calls) * 2**.

## Features

- Use backtracking to find all melds-meld candidates combinations.
- Extract melds and meld candidates separately for each suit.
- Since isolated tiles are not explicitly identified, the algorithm does not correctly calculate the shanten number for hands lacking sufficient isolated tiles.
  - For more details on hands lacking sufficient isolated tiles, see [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## References

- [向聴数を求めるアルゴリズム - あらの（一人）麻雀研究所](https://mahjong.ara.black/etc/shanten/index.htm)
