from __future__ import annotations
from itertools import product
from collections import namedtuple
from typing import Tuple, Literal, cast

RotationElem = Tuple[Literal[-1, 1], Literal[0, 1, 2]]
Rotation = Tuple[RotationElem, RotationElem, RotationElem]


def make_rotation(definition: str) -> Rotation:
    assert len(definition) == 6
    res = []
    for i in range(3):
        sign, axis = definition[i * 2 : i * 2 + 2]
        res.append(cast(RotationElem, (-1 if sign == "-" else 1, "xyz".index(axis))))
    return cast(Rotation, tuple(res))


ALL_ROTATIONS = [
    make_rotation(r)
    for r in (
        "+x+y+z",
        "-x+y-z",
        "+y-x+z",
        "-y-x-z",
        "+z+x+y",
        "-z+x-y",
        "+x-z+y",
        "-x+z+y",
        "+y-z-x",
        "-y+z-x",
        "+z-y+x",
        "-z+y+x",
        "+x-y-z",
        "-x-y+z",
        "+y+x-z",
        "-y+x+z",
        "+z-x-y",
        "-z-x+y",
        "+x+z-y",
        "-x-z-y",
        "+y+z+x",
        "-y-z+x",
        "+z+y-x",
        "-z-y-x",
    )
]

assert len(ALL_ROTATIONS) == 24


class Point(namedtuple("Point", "x,y,z")):
    __slots__ = ()

    def __add__(self, other: Point) -> Point:
        return Point(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: Point) -> Point:
        return Point(self.x - other.x, self.y - other.y, self.z - other.z)

    def rotate(self: Point, rotation: Rotation) -> Point:
        return Point(*(inv * self[idx] for inv, idx in rotation))

    @classmethod
    def parse(cls, number_str: str) -> Point:
        return cls(*map(int, number_str.split(",")))


class Scanner:
    INDEX = 0

    def __init__(self, points: set[Point]) -> None:
        self.index = Scanner.INDEX
        Scanner.INDEX += 1
        self.points = points
        self.fixed = False

    def match(self, other: Scanner) -> Point | None:
        other_points = list(other.points)
        for rotation in ALL_ROTATIONS:
            other_rotated = [p.rotate(rotation) for p in other_points]
            for l, r in product(self.points, other_rotated):
                other_ofs = l - r
                offset_points = [p + other_ofs for p in other_rotated]
                inter = set(offset_points) & self.points
                if len(inter) >= 12:
                    other.points = set(offset_points)
                    other.fixed = True
                    print("rotation:", rotation)
                    print("=== ORIG points:")
                    for p in sorted(inter):
                        other_idx = offset_points.index(p)
                        print(p, " <-> ", other_points[other_idx], " <(rot)> ", other_rotated[other_idx])
                    print("offset:", other_ofs)
                    print("===continuing===")
                    return other_ofs
        return None


SCANNERS = []

with open("19-test.txt") as f:
    while True:
        points: set[Point] = set()
        header = f.readline()
        if not header:
            break
        assert header.startswith("---")
        while line := f.readline().strip():
            points.add(Point.parse(line))
        SCANNERS.append(Scanner(points))

# SCANNERS[0].match(SCANNERS[1])
# import sys
# sys.exit(0)


def fix_all(scanners: list[Scanner]) -> None:
    scanners[0].fixed = True
    queue = [scanners[0]]
    remaining = scanners[1:]
    while queue:
        if not remaining:
            print("Failed to fix all")
        scanner = queue.pop(0)
        assert scanner.fixed
        for rem in remaining[:]:
            if ofs := scanner.match(rem):
                queue.append(rem)
                remaining.remove(rem)
                print(f"Scanner {rem.index} fixed at {ofs} of scanner {scanner.index}")


fix_all(SCANNERS)
