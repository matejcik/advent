from __future__ import annotations

import functools
import time
import typing as t

import numpy as np

D = t.TypeVar("D")


def timeit(limit: float = 2.0):
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            start = time.perf_counter()
            end = 0.0
            repetitions = 0
            while end - start < limit:
                repetitions += 1
                result = func(*args, **kwargs)
                end = time.perf_counter()
            avg_time = (end - start) / repetitions
            avg_time_ms = avg_time * 1000
            print(f"\t{func.__name__} took {avg_time_ms:.6f} ms")
            return result

        return wrapper

    return decorator


class Point(t.NamedTuple):
    x: int
    y: int

    def __add__(self, other) -> Point:
        return Point(self.x + other.x, self.y + other.y)

    def __sub__(self, other) -> Point:
        return Point(self.x - other.x, self.y - other.y)

    def __neg__(self) -> Point:
        return Point(-self.x, -self.y)

    def rotate_right(self) -> Point:
        return Point(-self.y, self.x)

    def rotate_left(self) -> Point:
        return Point(self.y, -self.x)


class Fastgrid:
    def __init__(self, grid: np.ndarray):
        self.grid = grid
        self.width = grid.shape[0]
        self.height = grid.shape[1]

    def __contains__(self, pos: Point) -> bool:
        return 0 <= pos.x < self.width and 0 <= pos.y < self.height

    def get(self, pos: Point, default: D = None) -> int | D:
        if not pos in self:
            return default
        return self.grid[*pos]

    def __getitem__(self, pos: Point) -> int:
        return self.grid[*pos]

    def __iter__(self) -> t.Iterator[Point]:
        return (Point(x, y) for y in range(self.height) for x in range(self.width))

    @classmethod
    def load(cls, lines: t.Iterator[bytes]) -> t.Self:
        rows = []
        for line in lines:
            if not line or line == "\n":
                break
            rows.append(line)

        array = np.array(rows)
        grid = array.view("i1").reshape((array.size), -1)
        return cls(grid.transpose()[:-1, :])


class Directions:
    UP = Point(0, -1)
    RIGHT = Point(1, 0)
    DOWN = Point(0, 1)
    LEFT = Point(-1, 0)


DIRECTIONS = [Directions.UP, Directions.RIGHT, Directions.DOWN, Directions.LEFT]
