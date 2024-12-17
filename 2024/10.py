from __future__ import annotations

import typing as t

import numpy as np
import pytest

import adventlib as adv

TRAILHEAD = ord(b"0")
SUMMIT = ord(b"9")


def score(trailhead: adv.Point, grid: adv.Fastgrid) -> int:
    queue = {trailhead}
    summits = set()
    while queue:
        point = queue.pop()
        elevation = grid[point]
        if elevation == SUMMIT:
            summits.add(point)
            continue
        for direction in adv.DIRECTIONS:
            new_point = point + direction
            new_elev = grid.get(new_point)
            if new_elev == elevation + 1:
                queue.add(new_point)
    return len(summits)


def rating(trailhead: adv.Point, grid: adv.Fastgrid) -> int:
    elevation = grid[trailhead]
    if elevation == SUMMIT:
        return 1
    total = 0
    for direction in adv.DIRECTIONS:
        new_point = trailhead + direction
        new_elev = grid.get(new_point)
        if new_elev == elevation + 1:
            total += rating(new_point, grid)
    return total


@adv.timeit()
def part1(grid: adv.Fastgrid) -> int:
    total = 0
    for point in grid:
        if grid[point] == TRAILHEAD:
            total += score(point, grid)

    return total


@adv.timeit()
def part2(grid: adv.Fastgrid) -> int:
    total = 0
    for point in grid:
        if grid[point] == TRAILHEAD:
            total += rating(point, grid)

    return total


PART1_VECTORS = (  # grid, result
    (
        """\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
""",
        2,
    ),
    (
        """\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
""",
        4,
    ),
    (
        """\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
""",
        3,
    ),
)


@pytest.mark.parametrize("input,expected", PART1_VECTORS)
def test_part1(input, expected) -> None:
    grid = adv.Fastgrid.load(l.encode() for l in input.splitlines(keepends=True))
    assert part1(grid) == expected


if __name__ == "__main__":
    with open("input/10.txt", "rb") as f:
        grid = adv.Fastgrid.load(f)

    print(f"Part 1: {part1(grid)=}")
    print(f"Part 2: {part2(grid)=}")
