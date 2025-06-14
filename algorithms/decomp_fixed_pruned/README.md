# Block Decomposition - Fixed and Pruned

## Devised by

[tomohxx](https://github.com/tomohxx)

## Overview

1. Extract a pair (Note: also consider the pattern where no pair is extracted).
2. Extract as many melds as possible.
3. Extract as many meld candidates as possible.  
   In this step, if the shanten number is less than or equal to **4 - (number of melds) - (number of pairs)**, further search for meld candidates is pruned.
4. Calculate the shanten number:

    - If the number of isolated tiles is lacking, add 1 to the shanten number.
    - Store the value if it is lower than other combinations.

5. Repeat for all possible combinations.

Constraint:

- **(number of melds) + (number of meld candidates) <= 4**

Formula for shanten number:  
**8 - (number of melds) * 2 - (number of meld candidates) - (number of pairs (0 or 1))**

- If there are calls, subtract **(number of calls) * 2**.

## Features

- An improved version of the algorithm originally devised by [麻雀C言語プログラム集](https://web.archive.org/web/20190616213620/http://cmj3.web.fc2.com/).
- If the number of melds and meld candidates reaches the maximum allowed, the search is terminated at that point, avoiding unnecessary further exploration.
- Applies a correction to the shanten number calculation when the hand lacks sufficient isolated tiles, enabling correct shanten number calculation in such cases.
  - For more details on hands lacking sufficient isolated tiles, see [ブロック分解方式向聴数計算アルゴリズムの精度の検証](https://zenn.dev/tomohxx/articles/aecace4e3a3bc1).

## References

- [tomohxx/shanten-test](https://github.com/tomohxx/shanten-test)
- [正確なブロック分解方式の向聴数計算アルゴリズムの提案](https://zenn.dev/tomohxx/articles/16c0d807218d2a)
