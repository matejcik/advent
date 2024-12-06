import functools
import time
import typing as t
from enum import Enum


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

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def rotate_right(self):
        return Point(-self.y, self.x)

    def rotate_left(self):
        return Point(self.y, -self.x)


class Directions(Enum):
    UP = Point(0, -1)
    DOWN = Point(0, 1)
    LEFT = Point(-1, 0)
    RIGHT = Point(1, 0)
