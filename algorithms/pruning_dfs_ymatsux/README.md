# Pruning DFS — ymatsux

## Origin

This pruning depth-first search was devised by Yoshitake Matsumoto (ymatsux).

## Algorithm

The search enumerates every legal combination of melds in nondecreasing meld-ID order. At each node it measures how many tiles are missing between the input hand and the partial winning-hand target. That value minus one is a lower bound. A branch is discarded when its lower bound cannot improve the best complete target found so far.

```text
search(target, melds_left, first_meld, upper_bound):
    if melds_left == 0:
        try every legal pair and update upper_bound
        return upper_bound

    for meld in melds[first_meld..]:
        add meld to target
        if target is legal and distance(hand, target) - 1 < upper_bound:
            search(target, melds_left - 1, meld.id, upper_bound)
        remove meld from target
```

The final shanten number is:

`sum(max(target[tile] - hand[tile], 0)) - 1`

## Properties

- Calculates the general-form shanten number only.
- Enumerates complete winning-hand targets rather than partial decompositions.
- Is exact because every legal general-form target is considered unless a valid lower bound proves that it cannot improve the result.
- Reuses the meld list for every calculation performed by one calculator instance.

## References

- <https://github.com/gimite/MjaiClients/blob/master/src/org/ymatsux/mjai/client/ShantensuUtil.java>
- [Pruning DFS — Mahjong Algorithm](https://tomohxx.github.io/mahjong-algorithm-book/dfs/)
