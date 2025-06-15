# Block Decomposition - Kobayashi

## Devised by

[Satoshi Kobayashi](https://github.com/kobalab)

## Overview

1. Extract a pair (also consider the pattern where no pair is extracted).
2. Extract melds, meld candidates, and isolated tiles from honors.
3. Extract melds, meld candidates, and isolated tiles from each suit:

    - Search for combinations of A and B as follows:

      - A: The combination that minimizes **(number of isolated tiles)**.
        - If the **(number of isolated tiles)** are the same, choose the one with the less **(number of meld candidates)**.
      - B: The combination that maximizes **(number of melds)**.
        - If the **(number of melds)** are the same, choose the one with the greater **(number of meld candidates)**.

4. Calculate the shanten number for all the combinations of A and B.

    - Store the value if it is lower than other combinations.

5. Repeat for all possible combinations.

### Constraint

- **(number of melds) + (number of meld candidates) <= 4**
- **(number of melds) + (number of meld candidates) + (number of pairs (0 or 1)) + (number of isolated tiles) <= 5**

### Formula for shanten number

**13 - (number of melds) * 3 - (number of meld candidates) * 2 - (number of pairs (0 or 1)) * 2 - (number of isolated tiles)**

If there are calls, subtract **(number of calls) * 3**.

## Features

- An improved version of the algorithm originally devised by [Ara](https://mahjong.ara.black/intro/selfintro.htm).
- The formula for shanten number is changed to support cases such as shanten numbers for specific yaku and other cases where not all tiles in the hand are used.
- After extracting meld from the hand, the algorithm uses properties of the count of tiles to determine the number of meld candidate, eliminating the need for explicit meld candidate extraction.
- Since isolated tiles are not explicitly identified, the algorithm does not correctly calculate the shanten number for hands lacking sufficient isolated tiles.
  - For more details on hands lacking sufficient isolated tiles, see [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## References

- [kobalab/majiang-core](https://github.com/kobalab/majiang-core)
- [対戦型麻雀ゲームAIのアルゴリズムと実装](https://www.amazon.co.jp/dp/4798067881)
