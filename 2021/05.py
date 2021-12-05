from dataclasses import dataclass
from collections import namedtuple, Counter
from typing import Iterable


Point = namedtuple("Point", "x, y")

@dataclass
class Line:
    a: Point
    b: Point

    @staticmethod
    def interpolate(start: int, end: int, dist: int) -> Iterable[int]:
        step = (end - start) / dist
        for i in range(dist + 1):
            yield round(start + step * i)

    def all_points(self) -> Iterable[Point]:
        dist = max(abs(self.a.x - self.b.x), abs(self.a.y - self.b.y))
        xs = self.interpolate(self.a.x, self.b.x, dist)
        ys = self.interpolate(self.a.y, self.b.y, dist)
        for x, y in zip(xs, ys):
            yield Point(x, y)

    def is_parallel_to_axis(self) -> bool:
        return self.a.x == self.b.x or self.a.y == self.b.y

    def dump(self) -> None:
        for point in self.all_points():
            print(f"{point.x},{point.y}, ", end="")
        print()


def print_grid(grid: Counter) -> None:
    maxx = maxy = 0
    for x, y in grid.keys():
        maxx = max(maxx, x)
        maxy = max(maxy, y)
    for y in range(maxy + 1):
        for x in range(maxx + 1):
            print(grid[(x, y)] or ".", end='')
        print()


def mkpoint(point: str) -> Point:
    x, y = point.split(',')
    return Point(int(x), int(y))


def count_intersections(lines: Iterable[Line]) -> int:
    grid = Counter(p for line in lines for p in line.all_points())
    # print_grid(grid)
    return sum(v > 1 for v in grid.values())


LINES = []


with open("05.txt") as f:
    for inp in f:
        l, _, r = inp.split()
        a = mkpoint(l)
        b = mkpoint(r)
        LINES.append(Line(a, b))

lines_pt1 = (line for line in LINES if line.is_parallel_to_axis())
print(f"Part 1: {count_intersections(lines_pt1)}")
print(f"Part 2: {count_intersections(LINES)}")
