from __future__ import annotations
from typing import NamedTuple, Any, Iterator
from contexttimer import Timer

import re

CUBES_50 = set()
INSTR_RE = re.compile(
    r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$"
)


class Voxel(NamedTuple):
    x: int
    y: int
    z: int


class Range(NamedTuple):
    left: int
    right: int

    def __contains__(self, x: Any) -> bool:
        if isinstance(x, Range):
            return x.left in self and x.right in self
        if isinstance(x, int):
            return self.left <= x <= self.right
        return False

    def overlaps(self, other: Range) -> bool:
        return self.left in other or self.right in other or other in self


class Cube(NamedTuple):
    top_near_left: Voxel
    bottom_far_right: Voxel

    def __contains__(self, thing: Any) -> bool:
        if isinstance(thing, Voxel):
            return self.top_near_left <= thing <= self.bottom_far_right
        if isinstance(thing, Cube):
            return thing.top_near_left in self and thing.bottom_far_right in self
        return False

    def intersection(self, other: Cube) -> Cube | None:
        if not self.overlaps(other):
            return None
        return Cube(
            max(self.top_near_left, other.top_near_left),
            min(self.bottom_far_right, other.bottom_far_right),
        )

    def range_x(self) -> Range:
        return Range(self.top_near_left.x, self.bottom_far_right.x)

    def range_y(self) -> Range:
        return Range(self.top_near_left.y, self.bottom_far_right.y)

    def range_z(self) -> Range:
        return Range(self.top_near_left.z, self.bottom_far_right.z)

    def overlaps(self, other: Cube) -> bool:
        return (
            self.range_x().overlaps(other.range_x())
            and self.range_y().overlaps(other.range_y())
            and self.range_z().overlaps(other.range_z())
        )

    def __iter__(self) -> Iterator[Voxel]:
        for x in range(self.top_near_left.x, self.bottom_far_right.x + 1):
            for y in range(self.top_near_left.y, self.bottom_far_right.y + 1):
                for z in range(self.top_near_left.z, self.bottom_far_right.z + 1):
                    yield Voxel(x, y, z)


CUBE_BOUNDS = Cube(Voxel(-50, -50, -50), Voxel(50, 50, 50))

INSTRUCTIONS: list[tuple[str, Cube]] = []

with open("22.txt") as f:
    for line in f:
        match = INSTR_RE.match(line)
        if not match:
            raise ValueError(f"Invalid instruction: {line}")
        op, *numbers = match.groups()
        cube = Cube(Voxel(*map(int, numbers[::2])), Voxel(*map(int, numbers[1::2])))
        INSTRUCTIONS.append((op, cube))


def toggle_in_bounds(bounds: Cube, instructions: list[tuple[str, Cube]]) -> set[Voxel]:
    toggled = set()
    for op, cube in instructions:
        if not cube.overlaps(bounds):
            continue
        for voxel in cube:
            if voxel not in bounds:
                continue
            if op == "on":
                toggled.add(voxel)
            elif op == "off":
                toggled.discard(voxel)
    return toggled


with Timer(factor=1000) as t:
    print(f"Part 1: {len(toggle_in_bounds(CUBE_BOUNDS, INSTRUCTIONS))}")
print(f"elapsed: {t.elapsed} ms")
