from collections import Counter
from typing import Tuple

GRID = Counter()

Point = Tuple[int, int]


def print_grid():
    maxx = maxy = 0
    for x, y in GRID.keys():
        maxx = max(maxx, x)
        maxy = max(maxy, y)
    for y in range(maxy + 1):
        for x in range(maxx + 1):
            print(GRID[(x, y)] or ".", end='')
        print()


def mkpoint(point: str) -> Point:
    x, y = point.split(',')
    return int(x), int(y)


def add_line(a: Point, b: Point) -> None:
    ax, ay = a
    bx, by = b
    if ax == bx:
        for y in range(min(ay, by), max(ay, by) + 1):
            GRID[(ax, y)] += 1
    elif ay == by:
        for x in range(min(ax, bx), max(ax, bx) + 1):
            GRID[(x, ay)] += 1


def add_diagonal_line(a: Point, b: Point) -> None:
    left, right = min(a, b), max(a, b)
    lx, ly = left
    rx, ry = right

    if lx == rx or ly == ry:
        return

    if ly > ry:
        dir_y = -1
    else:
        dir_y = 1

    for ofs in range(rx - lx + 1):
        GRID[(lx + ofs, ly + ofs * dir_y)] += 1


with open("05.txt") as f:
    for line in f:
        l, _, r = line.split()
        a = mkpoint(l)
        b = mkpoint(r)
        add_line(a, b)


intersection_count = sum(v > 1 for v in GRID.values())
print(f"Part 1: {intersection_count}")

with open("05.txt") as f:
    for line in f:
        l, _, r = line.split()
        a = mkpoint(l)
        b = mkpoint(r)
        add_diagonal_line(a, b)

intersection_count = sum(v > 1 for v in GRID.values())
print(f"Part 2: {intersection_count}")

# print_grid()
