# Block Decomposition - Pruned and Fixed

## Devised by

[tomohxx](https://github.com/tomohxx)

## Overview

(TODO)

Constraint:

- **(number of melds) + (number of meld candidates) <= 4**

Formula for shanten number:  
**8 - (number of melds) * 2 - (number of meld candidates) - (number of pairs (0 or 1))**

- If there are calls, subtract **(number of calls) * 2**.

## Features

- An improved version of the algorithm originally devised by [麻雀C言語プログラム集](https://web.archive.org/web/20190616213620/http://cmj3.web.fc2.com/).

## References

- [tomohxx/shanten-test](https://github.com/tomohxx/shanten-test)
- [正確なブロック分解方式の向聴数計算アルゴリズムの提案](https://zenn.dev/tomohxx/articles/16c0d807218d2a)
