from __future__ import annotations

import math
from collections import namedtuple
from functools import reduce
from operator import add
from typing import Iterable, TypeVar, Callable

from more_itertools import peekable
from contexttimer import Timer

T = TypeVar("T")
Element = namedtuple("Element", "depth, value")


class SnailfishNumber:
    EXPLODE_DEPTH = 5

    def __init__(self, number: Iterable[Element]) -> None:
        self.number = list(number)

    @staticmethod
    def _parse(number_str: str) -> Iterable[Element]:
        depth = 0
        digits = ""

        def make_elem():
            nonlocal digits
            if not digits:
                return
            elem = Element(depth, int(digits))
            digits = ""
            yield elem

        for char in number_str:
            if char == "[":
                depth += 1
            elif char == "]":
                yield from make_elem()
                depth -= 1
            elif char.isdigit():
                digits += char
            elif char == ",":
                yield from make_elem()
            else:
                raise ValueError(f"invalid character: {char}")

    @classmethod
    def parse(cls, number_str: str) -> SnailfishNumber:
        return cls(cls._parse(number_str))

    def explode(self) -> bool:
        for idx, elem in enumerate(self.number):
            if elem.depth < self.EXPLODE_DEPTH:
                continue
            assert elem.depth == self.EXPLODE_DEPTH, "too deeply nested?"
            assert self.number[idx + 1].depth == self.EXPLODE_DEPTH, "invalid pair"
            if idx > 0:
                left = self.number[idx - 1]
                self.number[idx - 1] = Element(left.depth, left.value + elem.value)
            if idx < len(self.number) - 2:
                # idx must be second-to-last for this to hit, because the last element
                # is the other part of the pair
                other = self.number[idx + 1]
                right = self.number[idx + 2]
                self.number[idx + 2] = Element(right.depth, other.value + right.value)
            self.number[idx : idx + 2] = [Element(elem.depth - 1, 0)]
            return True
        return False

    def split(self) -> bool:
        for idx, elem in enumerate(self.number):
            if elem.value <= 9:
                continue
            self.number[idx : idx + 1] = [
                Element(elem.depth + 1, math.floor(elem.value / 2)),
                Element(elem.depth + 1, math.ceil(elem.value / 2)),
            ]
            return True
        return False

    def reduce(self) -> None:
        while True:
            if self.explode():
                continue
            if self.split():
                continue
            break

    def __add__(self, other: SnailfishNumber) -> SnailfishNumber:
        if not self:
            return other
        new_number = SnailfishNumber(
            Element(e.depth + 1, e.value) for e in self.number + other.number
        )
        new_number.reduce()
        return new_number

    @classmethod
    def _dfs_visit(
        cls,
        number: peekable[Element],
        node_visitor: Callable[[T, T], T],
        leaf_visitor: Callable[[int], T],
        depth: int,
    ) -> T:
        def next_item() -> T:
            elem = number.peek()
            if elem.depth == depth:
                next(number)
                return leaf_visitor(elem.value)
            else:
                return cls._dfs_visit(number, node_visitor, leaf_visitor, depth + 1)

        left = next_item()
        right = next_item()
        return node_visitor(left, right)

    def dfs_visit(
        self, node_visitor: Callable[[T, T], T], leaf_visitor: Callable[[int], T] = int
    ) -> T:
        assert self, "empty"
        p = peekable(self.number)
        return self._dfs_visit(p, node_visitor, leaf_visitor, 1)

    def __bool__(self) -> bool:
        return bool(self.number)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, SnailfishNumber):
            return False
        return self.number == other.number

    def __str__(self) -> str:
        return self.dfs_visit(lambda x, y: f"[{x},{y}]", str)


def magnitude(left: int, right: int) -> int:
    return 3 * left + 2 * right


def test_explode():
    number = SnailfishNumber.parse("[[[[[9,8],1],2],3],4]")
    assert number.explode() is True
    assert number == SnailfishNumber.parse("[[[[0,9],2],3],4]")


def test_split():
    number = SnailfishNumber.parse("[[[[0,7],4],[15,[0,13]]],[1,1]]")
    assert number.split() is True
    assert number == SnailfishNumber.parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")


def test_reduce():
    number = SnailfishNumber.parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
    number.reduce()
    assert number == SnailfishNumber.parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")


def test_magnitude():
    number = SnailfishNumber.parse("[[1,2],[[3,4],5]]")
    mag = number.dfs_visit(magnitude)
    assert mag == 143


def test_addition():
    a = SnailfishNumber.parse("[[[[4,3],4],4],[7,[[8,4],9]]]")
    b = SnailfishNumber.parse("[1,1]")
    assert a + b == SnailfishNumber.parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")


if __name__ == "__main__":
    NUMBERS: list[SnailfishNumber] = []
    with open("18.txt") as f:
        for line in f:
            NUMBERS.append(SnailfishNumber.parse(line.strip()))

    sum_of_all: SnailfishNumber = reduce(add, NUMBERS)
    print(f"Part 1: {sum_of_all.dfs_visit(magnitude)}")

    with Timer(factor=1000) as t:
        max_mag = 0
        for i, a in enumerate(NUMBERS):
            for j, b in enumerate(NUMBERS):
                if i == j:
                    continue
                mag = (a + b).dfs_visit(magnitude)
                max_mag = max(mag, max_mag)

    print(f"Part 2: {max_mag} ({t.elapsed} ms)")
