from __future__ import annotations
from dataclasses import dataclass
from typing import Tuple, Literal

Pixel = Tuple[int, int]
Value = Literal[0, 1]


def to_value(c: str) -> Value:
    return 0 if c == "." else 1


@dataclass
class Grid:
    algorithm: list[Value]
    pixels: dict[Pixel, Value]
    infinity: Value = 0

    def dimensions(self) -> Tuple[int, int, int, int]:
        min_x = max_x = min_y = max_y = 0
        for x, y in self.pixels:
            min_x = min(min_x, x)
            max_x = max(max_x, x)
            min_y = min(min_y, y)
            max_y = max(max_y, y)
        return min_x, max_x, min_y, max_y

    def new_value(self, px: int, py: int) -> Value:
        number = 0
        for y in (-1, 0, 1):
            for x in (-1, 0, 1):
                val = self.pixels.get((px + x, py + y), self.infinity)
                number = (number << 1) + val
        return self.algorithm[number]

    def enhance(self) -> None:
        min_x, max_x, min_y, max_y = self.dimensions()
        new_grid = {}
        for x in range(min_x - 1, max_x + 2):
            for y in range(min_y - 1, max_y + 2):
                new_grid[x, y] = self.new_value(x, y)

        self.pixels = new_grid
        self.infinity = self.algorithm[-1 * self.infinity]

    def count_lit(self) -> int:
        if self.infinity == 1:
            raise ValueError("Infinity is lit")
        return sum(self.pixels.values())

    def __str__(self) -> str:
        min_x, max_x, min_y, max_y = self.dimensions()
        res = []
        for y in range(min_y, max_y + 1):
            line = ""
            for x in range(min_x, max_x + 1):
                line += "." if self.pixels[x, y] == 0 else "#"
            res.append(line)
        return "\n".join(res)


with open("20.txt") as f:
    algorithm: list[Value] = [to_value(c) for c in f.readline().strip()]
    assert f.readline() == "\n"
    y = 0
    pixels: dict[Pixel, Value] = {}
    for line in f:
        for x, val in enumerate(line.strip()):
            pixels[x, y] = to_value(val)
        y += 1

grid = Grid(algorithm, pixels)
grid.enhance()
grid.enhance()

print(f"Part 1: {grid.count_lit()}")

for _ in range(48):
    grid.enhance()

print(f"Part 2: {grid.count_lit()}")
