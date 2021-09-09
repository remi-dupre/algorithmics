#!/usr/bin/env python3
from functools import lru_cache
from typing import List

WALK_MAX = 3
LENGTH = 15
NB_PLAYERS = 4


@lru_cache
def pos_at_turn(turn: int) -> List[float]:
    """
    Position distribution of a larva at a given turn.
    """
    if turn == 0:
        return [1.0, *(0.0 for _ in range(LENGTH))]

    prev = pos_at_turn(turn - 1)

    curr = [
        # P(x) = (P(x-1) + P(x-2) + P(x-3)) / 3
        sum(prev[y] for y in range(max(0, x - 3), x)) / 3
        for x in range(LENGTH)
    ]

    # Stay at the last position when the end is reached
    curr.append(1 - sum(curr))
    return curr


def win_at_turn(turn: int) -> float:
    """
    Probability that a larva already reached the end at a given turn.
    """
    return pos_at_turn(turn)[-1]


if __name__ == "__main__":
    # The first larva wins at a given turn if no larva already won at previous
    # turn, and it reaches the end at this turn.
    p_win = sum(
        win_at_turn(turn) * (1 - win_at_turn(turn - 1)) ** NB_PLAYERS
        for turn in range(1, LENGTH + 1)
    )

    print(f"Probability that the first larva wins: {p_win:.2}")
