# handgen

Generates the fixed benchmark hands used by the shanten algorithm crates in this workspace.

## Datasets

The generator writes 10,000 hands for each of the following distributions:

- `normal`: all 34 tile types
- `half_flush`: one randomly selected suit and all honor tiles
- `full_flush`: one randomly selected suit
- `thirteen_orphans`: the 13 terminal and honor tile types

Each hand contains 14 tile IDs sampled without replacement from a wall with
four copies of every available tile type. The random number generator uses the
fixed seed `42`, so running the program again produces the same datasets.

## Usage

Run the generator from the workspace root:

```sh
cargo run --package handgen
```

This overwrites the following files under `resources/`:

- `hands_normal_10000.txt`
- `hands_half_flush_10000.txt`
- `hands_full_flush_10000.txt`
- `hands_thirteen_orphans_10000.txt`

Each line contains 14 space-separated tile IDs in the range 0–33. Tile IDs use
the following mapping. The suffixes `m`, `p`, `s`, and `z` denote characters
(萬子), circles (筒子), bamboo (索子), and honors (字牌), respectively.

| ID   | 0   | 1   | 2   | 3   | 4   | 5   | 6   | 7   | 8   |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile | 1m  | 2m  | 3m  | 4m  | 5m  | 6m  | 7m  | 8m  | 9m  |

| ID   | 9   | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 17  |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile | 1p  | 2p  | 3p  | 4p  | 5p  | 6p  | 7p  | 8p  | 9p  |

| ID   | 18  | 19  | 20  | 21  | 22  | 23  | 24  | 25  | 26  |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Tile | 1s  | 2s  | 3s  | 4s  | 5s  | 6s  | 7s  | 8s  | 9s  |

| ID   | 27        | 28         | 29        | 30         | 31         | 32         | 33       |
| ---- | --------- | ---------- | --------- | ---------- | ---------- | ---------- | -------- |
| Tile | East (1z) | South (2z) | West (3z) | North (4z) | White (5z) | Green (6z) | Red (7z) |
