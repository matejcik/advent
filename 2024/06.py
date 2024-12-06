from dataclasses import dataclass, field
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
    print(f"Part 2: {guard_loop(grid)=}")
