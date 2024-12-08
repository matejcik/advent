from dataclasses import dataclass
import typing as t

import numpy as np

from adventlib import timeit, Point


@dataclass
class Grid:
    width: int
    height: int
    antenna_sets: dict[str, set[Point]]

    @classmethod
    def parse(cls, lines: t.Iterable[str]) -> t.Self:
        antenna_sets = {}
        for y, line in enumerate(lines):
            for x, c in enumerate(line.rstrip()):
                if c == ".":
                    continue
                antenna_sets.setdefault(c, set()).add(Point(x, y))
        return cls(x + 1, y + 1, antenna_sets)

    def __contains__(self, pos: Point) -> bool:
        return 0 <= pos.x < self.width and 0 <= pos.y < self.height


def count_antinodes_part1(grid: Grid, antenna_set: set[Point]) -> set[Point]:
    arr = np.array(list(antenna_set))
    arr2 = arr * 2
    unique_antinodes = set()
    for i in range(1, len(arr)):
        shifted = np.roll(arr, i, axis=0)
        antinodes = arr2 - shifted
        unique_antinodes.update(p for x, y in antinodes if (p := Point(x, y)) in grid)
    return unique_antinodes


def count_antinodes_part2(grid: Grid, antenna_set: set[Point]) -> set[Point]:
    arr = np.array(list(antenna_set))
    unique_antinodes = set()
    for i in range(1, len(arr)):
        shifted = np.roll(arr, i, axis=0)
        vectors = arr - shifted
        gcds = np.gcd(vectors[:, 0], vectors[:, 1])[0]
        step_vectors = vectors // gcds
        for (x, y), (dx, dy) in zip(arr, step_vectors):
            p = Point(x, y)
            dp = Point(dx, dy)
            up = p
            while up in grid:
                unique_antinodes.add(up)
                up = up - dp
            down = p
            while down in grid:
                unique_antinodes.add(down)
                down = down + dp

    return unique_antinodes


def test_part1_full():
    EXAMPLE = """\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"""
    grid = Grid.parse(EXAMPLE.splitlines())
    assert part1(grid) == 14


@timeit()
def part1(grid: Grid) -> int:
    all_uniques = set()
    for antenna_set in grid.antenna_sets.values():
        all_uniques.update(count_antinodes_part1(grid, antenna_set))
    return len(all_uniques)


@timeit()
def part2(grid: Grid) -> int:
    all_uniques = set()
    for antenna_set in grid.antenna_sets.values():
        all_uniques.update(count_antinodes_part2(grid, antenna_set))
    return len(all_uniques)


if __name__ == "__main__":
    with open("input/08.txt") as f:
        grid = Grid.parse(f)
    print(f"Part 1: {part1(grid)=}")
    print(f"Part 2: {part2(grid)=}")
