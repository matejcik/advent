from dataclasses import dataclass
import typing as t

from adventlib import timeit, Directions, Point


@dataclass(frozen=True)
class Grid:
    obstacles: frozenset[Point]
    guard_pos: Point
    guard_dir: Point
    width: int
    height: int

    DIRECTIONS: t.ClassVar[dict[str, Point]] = {
        "v": Directions.DOWN.value,
        "^": Directions.UP.value,
        "<": Directions.LEFT.value,
        ">": Directions.RIGHT.value,
    }

    @classmethod
    def parse(cls, input: t.Iterable[str]) -> t.Self:
        obstacles = set()
        guard_pos: Point | None = None
        guard_dir: Point | None = None
        x = y = 0
        for y, line in enumerate(input):
            for x, c in enumerate(line):
                if c == "#":
                    obstacles.add(Point(x, y))
                elif c in cls.DIRECTIONS:
                    guard_pos = Point(x, y)
                    guard_dir = cls.DIRECTIONS[c]
        assert guard_pos is not None and guard_dir is not None
        # we don't strip the line which ends with a newline
        # therefore the actual width is the current x, _without_ trailing newline
        return cls(frozenset(obstacles), guard_pos, guard_dir, x, y + 1)

    def __contains__(self, pos: Point) -> bool:
        return 0 <= pos.x < self.width and 0 <= pos.y < self.height


class Ray(t.NamedTuple):
    """Straight axis-parallel ray on a grid."""

    a: Point
    b: Point
    direction: Point
    exits: bool

    @classmethod
    def cast(cls, start: Point, direction: Point, grid: Grid) -> t.Self:
        """Cast a ray from a point in a given direction."""
        next = start
        while next in grid:
            end = next
            next = end + direction
            if next in grid.obstacles:
                return cls(start, end, direction, False)
        return cls(start, end, direction, True)

    def __len__(self) -> int:
        # assuming that the ray is axis-parallel, so either X or Y is going to be zero
        return abs(self.b.x - self.a.x) + abs(self.b.y - self.a.y)


def intersect(left: Ray, right: Ray) -> bool:
    if left.a == right.b or left.b == right.a:
        return False
    if (left.direction.x == 0) == (right.direction.x == 0):
        # both are on the same axis. there is no intersection if we got these rays
        # by casting in the same grid.
        return False
    # order the rays by which is parallel to the X axis
    x_ray, y_ray = (left, right) if left.direction.y == 0 else (right, left)
    x_min = min(x_ray.a.x, x_ray.b.x)
    x_max = max(x_ray.a.x, x_ray.b.x)
    y_min = min(y_ray.a.y, y_ray.b.y)
    y_max = max(y_ray.a.y, y_ray.b.y)
    return x_min <= y_ray.a.x <= x_max and y_min <= x_ray.a.y <= y_max


@dataclass
class GridWalk:
    grid: Grid
    walked_positions: set[tuple[Point, Point]]
    current_pos: Point
    current_dir: Point
    additional_obstacle: Point

    @classmethod
    def new(cls, grid: Grid) -> t.Self:
        return cls(grid, set(), grid.guard_pos, grid.guard_dir, Point(-1, -1))

    def fork(self, additional_obstacle: Point) -> t.Self:
        return self.__class__(
            self.grid,
            self.walked_positions,
            self.current_pos,
            self.current_dir,
            additional_obstacle,
        )

    def next_pos(self) -> Point:
        """Take a step or rotate if you can't. Return the next position.

        We update the current_dir in-place, but not current_pos; in all our usecases, we
        want the caller to decide whether to take the next step, but we never need to
        decide whether or not to rotate.
        """
        next_pos = self.current_pos + self.current_dir
        if next_pos in self.grid.obstacles or next_pos == self.additional_obstacle:
            self.current_dir = self.current_dir.rotate_right()
            return self.current_pos
        return next_pos

    def is_loop(self) -> bool:
        looped_positions = set()
        while self.current_pos in self.grid:
            self.current_pos = self.next_pos()
            pos = (self.current_pos, self.current_dir)
            if pos in self.walked_positions or pos in looped_positions:
                return True
            looped_positions.add(pos)
        return False


@dataclass
class GridWalkRayCast:
    grid: Grid


@timeit()
def guard_walk(grid: Grid) -> int:
    positions = set()
    pos = grid.guard_pos
    dir = grid.guard_dir
    while pos in grid:
        positions.add(pos)
        next_pos = pos + dir
        if next_pos in grid.obstacles:
            dir = dir.rotate_right()
        else:
            pos = next_pos
    return len(positions)


@timeit()
def guard_walk_raycast(grid: Grid) -> int:
    xrays = []
    yrays = []
    pos = grid.guard_pos
    dir = grid.guard_dir
    while True:
        ray = Ray.cast(pos, dir, grid)
        if dir.x == 0:
            xrays.append(ray)
        else:
            yrays.append(ray)
        if ray.exits:
            break
        pos = ray.b
        dir = dir.rotate_right()

    # sum the total length of the rays
    total = 1 + sum(map(len, xrays)) + sum(map(len, yrays))
    # # subtract all nontrivial interesctions
    for xray in xrays:
        intersections = sum(intersect(xray, r) for r in yrays)
        total -= intersections

    return total


@timeit()
def guard_loop(grid: Grid) -> int:
    walk = GridWalk.new(grid)
    total = 0
    obstacles_tested = set()
    walk.walked_positions.add((walk.current_pos, walk.current_dir))
    while True:
        next_pos = walk.next_pos()
        if next_pos not in walk.grid:
            break
        if next_pos != walk.current_pos and next_pos not in obstacles_tested:
            obstacles_tested.add(next_pos)
            fork = walk.fork(next_pos)
            if fork.is_loop():
                total += 1
        walk.current_pos = next_pos
        walk.walked_positions.add((walk.current_pos, walk.current_dir))
    return total


@timeit()
def guard_loop_raycast(grid: Grid) -> int:
    pass


def test_guard_loop():
    EXAMPLE = """\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""
    grid = Grid.parse(EXAMPLE.splitlines(keepends=True))
    assert guard_loop(grid) == 6


if __name__ == "__main__":
    with open("input/06.txt") as f:
        grid = Grid.parse(f)
    print(f"Part 1: {guard_walk(grid)=}")
    print(f"Part 1 alt: {guard_walk_raycast(grid)=}")
    print(f"Part 2: {guard_loop(grid)=}")
