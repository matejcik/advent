from __future__ import annotations

import time
from typing import Tuple, Iterable

Pixel = Tuple[int, int]


class GridGraph:
    def __init__(self, grid: dict[Pixel, int]) -> None:
        self.grid = grid

    def get_shape(self):
        return max(x for x, _ in self.grid) + 1, max(y for _, y in self.grid) + 1

    def neigbors(self, pixel) -> Iterable[Pixel]:
        for ox, oy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            x, y = pixel
            npx = x + ox, y + oy
            if npx in self.grid:
                yield npx

    @classmethod
    def from_str(cls, grid_str: str) -> "GridGraph":
        grid = {}
        for y, line in enumerate(grid_str.splitlines()):
            for x, c in enumerate(line):
                grid[(x, y)] = int(c)
        return cls(grid)

    def lowest_risk_path(self) -> int:
        risk_queue: list[tuple[int, Pixel]] = [(0, (0, 0))]
        risks: dict[Pixel, int] = {(0, 0): 0}
        width, height = self.get_shape()
        while risk_queue:
            risk_queue.sort()
            risk, pixel = risk_queue.pop(0)
            if pixel == (width - 1, height - 1):
                return risk

            for n in self.neigbors(pixel):
                new_risk = risk + self.grid[n]
                if n not in risks or new_risk < risks[n]:
                    risks[n] = new_risk
                    risk_queue.append((new_risk, n))
        raise Exception("No path found")

    def enlarge(self, factor: int) -> None:
        def wrap_value(i: int, add: int) -> int:
            if i + add > 9:
                return (i + add) % 10 + 1
            else:
                return i + add

        width, height = self.get_shape()
        for y in range(height):
            for x in range(width):
                for add in range(1, factor):
                    self.grid[(x + width * add, y)] = wrap_value(self.grid[(x, y)], add)
        width, height = self.get_shape()
        for x in range(width):
            for y in range(height):
                for add in range(1, factor):
                    self.grid[(x, y + height * add)] = wrap_value(
                        self.grid[(x, y)], add
                    )

        width, height = self.get_shape()
        for x in range(width):
            for y in range(height):
                assert (x, y) in self.grid, f"{x}, {y} not in grid"


with open("15.txt") as f:
    grid = GridGraph.from_str(f.read())
    start = time.time()
    print(f"Part 1: {grid.lowest_risk_path()}")
    end = time.time()
    print(f"Part 1 took: {end - start:.3f}s")

    grid.enlarge(5)
    start = time.time()
    print(f"Part 2: {grid.lowest_risk_path()}")
    end = time.time()
    print(f"Part 2 took: {end - start:.3f}s")
