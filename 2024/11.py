from functools import cache
import typing as t

import pytest

from adventlib import timeit


@cache
def evolve(number: int, blinks: int) -> int:
    if blinks == 0:
        return 1
    if number == 0:
        return evolve(1, blinks - 1)
    nstr = str(number)
    if len(nstr) % 2 == 0:
        half = len(nstr) // 2
        left = evolve(int(nstr[:half]), blinks - 1)
        right = evolve(int(nstr[half:]), blinks - 1)
        return left + right
    return evolve(number * 2024, blinks - 1)


@pytest.mark.parametrize(
    "input, steps, expected",
    (
        (0, 1, 1),
        (1, 1, 1),
        (10, 1, 2),
        (100, 1, 1),
        (1000, 1, 2),
        (0, 2, 1),
        (0, 3, 2),
    ),
)
def test_evolve(input, steps, expected) -> None:
    assert evolve(input, steps) == expected

@timeit()
def evolve_input(input: t.Iterable[int], blinks: int) -> int:
    return sum(evolve(n, blinks) for n in input)


if __name__ == "__main__":
    with open("input/11.txt") as f:
        input = [int(n) for n in f.read().strip().split()]
    print(f"Part 1: {evolve_input(input, 25)=}")
    print(f"Part 2: {evolve_input(input, 75)=}")
