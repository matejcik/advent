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
            return bool(self and x and x.left in self and x.right in self)
        if isinstance(x, int):
            return self.left <= x <= self.right
        return False

    def __and__(self, other: Range) -> Range:
        if not self or not other:
            return Range(1, -1)
        return Range(max(self.left, other.left), min(self.right, other.right))

    def __bool__(self) -> bool:
        return self.left <= self.right

    def overlaps(self, other: Range) -> bool:
        return bool(self & other)

    def __len__(self) -> int:
        return max(0, self.right - self.left + 1)

    def __repr__(self) -> str:
        return f"<{self.left},{self.right}>"

    def slice(self, x: int) -> tuple[Range, Range]:
        if x not in self:
            raise ValueError(f"{x} not in {self}")
        return Range(self.left, x), Range(x + 1, self.right)

    def parts(self, other: Range) -> Iterator[Range]:
        if not other in self:
            raise ValueError(f"{other} not in {self}")
        if other.left > self.left:
            yield Range(self.left, other.left - 1)
        yield other
        if other.right < self.right:
            yield Range(other.right + 1, self.right)


class Cube(NamedTuple):
    x: Range
    y: Range
    z: Range

    def __contains__(self, thing: Any) -> bool:
        if isinstance(thing, (Voxel, Cube)):
            return thing.x in self.x and thing.y in self.y and thing.z in self.z
        return False

    def __and__(self, other: Cube) -> Cube:
        return Cube(
            self.x & other.x,
            self.y & other.y,
            self.z & other.z,
        )

    def __bool__(self) -> bool:
        return bool(self.x) and bool(self.y) and bool(self.z)

    def overlaps(self, other: Cube) -> bool:
        return bool(self & other)

    def __len__(self) -> int:
        return len(self.x) * len(self.y) * len(self.z)

    @staticmethod
    def max(*cubes: Cube) -> Cube:
        return max(cubes, key=lambda cube: len(cube))

    @staticmethod
    def min(*cubes: Cube) -> Cube:
        return min(cubes, key=lambda cube: len(cube))

    def slice(self, **kwargs) -> tuple[Cube, Cube]:
        if len(kwargs) > 1:
            raise ValueError("Can only slice one dimension at a time")
        key, value = next(iter(kwargs.items()))
        if key not in "xyz":
            raise ValueError("Can only slice on x, y, or z")
        left = {}
        right = {}
        for coord in "xyz":
            range = getattr(self, coord)
            if coord == key:
                left[coord], right[coord] = range.slice(value)
            else:
                left[coord] = right[coord] = range
        return Cube(**left), Cube(**right)

    def slice_remove(self, other: Cube) -> Iterator[Cube]:
        if not isinstance(other, Cube):
            raise TypeError(f"Can only subtract a Cube from a Cube, not {type(other)}")
        if other not in self:
            raise ValueError(f"Can only subtract sub-cubes from super-cubes")
        if other == self:
            return
        assert self & other, "wat"

        remains = self
        for coord in "xyz":
            kwargs = {"x": remains.x, "y": remains.y, "z": remains.z}
            my_range = kwargs[coord]
            other_range = getattr(other, coord)
            for part in my_range.parts(other_range):
                kwargs[coord] = part
                slice = Cube(**kwargs)
                if part == other_range:
                    remains = slice
                else:
                    yield slice
        assert remains == other, f"after slicing {remains} is not equal to {other}"


class Space:
    def __init__(self) -> None:
        self.cubes: set[Cube] = set()

    def add(self, cube: Cube) -> None:
        add_queue = [cube]
        while add_queue:
            new_cube = add_queue.pop(0)
            for known_cube in self.cubes:
                if new_cube in known_cube:
                    break
                inter = known_cube & new_cube
                if inter:
                    add_queue.extend(new_cube.slice_remove(inter))
                    break
            else:
                self.cubes.add(new_cube)

    def remove(self, cube: Cube) -> None:
        new_cubes = set()
        for known_cube in self.cubes:
            inter = known_cube & cube
            if not inter:
                new_cubes.add(known_cube)
            else:
                new_cubes.update(known_cube.slice_remove(inter))
        self.cubes = new_cubes

    def __len__(self) -> int:
        return sum(len(cube) for cube in self.cubes)


def test_range_parts():
    big = Range(-10, 10)
    small = Range(-5, 5)

    big_parts = list(big.parts(small))
    assert small in big_parts
    assert big_parts == [Range(-10, -6), Range(-5, 5), Range(6, 10)]

    left = Range(-10, 5)
    left_parts = list(big.parts(left))
    assert left in left_parts
    assert left_parts == [Range(-10, 5), Range(6, 10)]

    right = Range(5, 10)
    right_parts = list(big.parts(right))
    assert right in right_parts
    assert right_parts == [Range(-10, 4), Range(5, 10)]

    big_big = list(big.parts(big))
    assert big_big == [big]


CUBE_BOUNDS = Cube(Range(-50, 50), Range(-50, 50), Range(-50, 50))

INSTRUCTIONS: list[tuple[str, Cube]] = []

with open("22.txt") as f:
    for line in f:
        match = INSTR_RE.match(line)
        if not match:
            raise ValueError(f"Invalid instruction: {line}")
        op, *numbers = match.groups()
        cube = Cube(
            Range(*map(int, numbers[:2])),
            Range(*map(int, numbers[2:4])),
            Range(*map(int, numbers[4:])),
        )
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


def toggle_with_space(bounds: Cube | None, instructions: list[tuple[str, Cube]]) -> Space:
    toggled = Space()
    for op, cube in instructions:
        if bounds and not cube.overlaps(bounds):
            continue
        if op == "on":
            toggled.add(cube)
        elif op == "off":
            toggled.remove(cube)
    return toggled


# with Timer(factor=1000) as t:
#     print(f"Part 1: {len(toggle_in_bounds(CUBE_BOUNDS, INSTRUCTIONS))}")
# print(f"elapsed: {t.elapsed} ms")

with Timer(factor=1000) as t:
    print(f"Part 1 with space: {len(toggle_with_space(CUBE_BOUNDS, INSTRUCTIONS))}")
print(f"elapsed: {t.elapsed} ms")

with Timer(factor=1000) as t:
    space = toggle_with_space(None, INSTRUCTIONS)
    print(f"Part 2: {len(space)}")
print(f"elapsed: {t.elapsed} ms")
