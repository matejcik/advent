from __future__ import annotations

from enum import Enum
from typing import Tuple

Pixel = Tuple[int, int]


class Cucumber(Enum):
    Down = (0, 1)
    Right = (1, 0)


class Grid:
    def __init__(self):
        self.grid: dict[Pixel, Cucumber] = {}
        self.width = 0
        self.height = 0

    def add(self, x: int, y: int, cucumber: Cucumber):
        self.grid[(x, y)] = cucumber
        self.width = max(self.width, x + 1)
        self.height = max(self.height, y + 1)

    def step(self) -> bool:
        changed = False
        for row in range(self.height):
            occupied = (0, row) in self.grid
            should_skip_first = not occupied
            for x in range(self.width, 1 * should_skip_first, -1):
                px = x - 1, row
                current = self.grid.get(px)
                if not occupied and current is Cucumber.Right:
                    self.grid[x % self.width, row] = Cucumber.Right
                    del self.grid[px]
                    changed = True
                occupied = current is not None

        for col in range(self.width):
            occupied = (col, 0) in self.grid
            should_skip_first = not occupied
            for y in range(self.height, 1 * should_skip_first, -1):
                px = col, y - 1
                current = self.grid.get(px)
                if not occupied and current is Cucumber.Down:
                    self.grid[col, y % self.height] = Cucumber.Down
                    del self.grid[px]
                    changed = True
                occupied = current is not None
            
        return changed

    CHARS = {Cucumber.Down: "v", Cucumber.Right: ">", None: "."}

    def _row_str(self, row: int) -> str:
        return "".join(self.CHARS[self.grid.get((x, row))] for x in range(self.width))

    def _col_str(self, col: int) -> str:
        return "".join(self.CHARS[self.grid.get((col, y))] for y in range(self.height))

    def __str__(self) -> str:
        return "\n".join(self._row_str(row) for row in range(self.height))

    def __repr__(self) -> str:
        return f"Grid(width={self.width}, height={self.height})"


GRID = Grid()

with open("25.txt") as f:
    for y, line in enumerate(f):
        for x, char in enumerate(line.strip()):
            if char == "v":
                GRID.add(x, y, Cucumber.Down)
            elif char == ">":
                GRID.add(x, y, Cucumber.Right)
        GRID.width = x + 1
    GRID.height = y + 1

steps = 1
while GRID.step():
    steps += 1

print(f"Steps: {steps}")
