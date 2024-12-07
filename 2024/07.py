import typing as t

from adventlib import timeit


class Entry(t.NamedTuple):
    expected: int
    formula: list[int]

    def pop_with(self, expected: int) -> t.Self:
        return self.__class__(expected, self.formula[:-1])

    def test_valid_part1(self) -> bool:
        if not self.formula:
            return self.expected == 0

        last = self.formula[-1]
        if self.expected % last == 0:
            if self.pop_with(self.expected // last).test_valid_part1():
                return True
        return self.pop_with(self.expected - last).test_valid_part1()

    def test_valid_part2(self) -> bool:
        if not self.formula:
            return self.expected == 0

        last = self.formula[-1]
        # test for concat
        # 1. find nearest multiple of 10 above `last`
        i = last
        nearest_multiple = 1
        while i > 0:
            nearest_multiple *= 10
            i //= 10

        if self.expected % nearest_multiple == last:
            if self.pop_with(self.expected // nearest_multiple).test_valid_part2():
                return True

        # test for part1 operators
        if self.expected % last == 0:
            if self.pop_with(self.expected // last).test_valid_part2():
                return True
        return self.pop_with(self.expected - last).test_valid_part2()

    @classmethod
    def parse(cls, line: str) -> t.Self:
        expected_str, formula_str = line.rstrip().split(":")
        numbers = formula_str.strip().split()
        return cls(int(expected_str), [int(x) for x in numbers])


@timeit()
def part1(entries: list[Entry]) -> int:
    return sum(e.expected for e in entries if e.test_valid_part1())


@timeit()
def part2(entries: list[Entry]) -> int:
    return sum(e.expected for e in entries if e.test_valid_part2())


def test_part2():
    EXAMPLE = """\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"""
    entries = [Entry.parse(line) for line in EXAMPLE.splitlines()]
    for entry in entries:
        if entry.test_valid_part1():
            print("part1 ok:", entry)
        if entry.test_valid_part2():
            print("part2 ok:", entry)
    assert part2(entries) == 11387


if __name__ == "__main__":
    with open("input/07.txt") as f:
        entries = [Entry.parse(line) for line in f]
    print(f"Part 1: {part1(entries)=}")
    print(f"Part 2: {part2(entries)=}")
