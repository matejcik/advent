from dataclasses import dataclass, field
from collections import defaultdict

import adventlib

def _dict_of_sets() -> dict[int, set[int]]:
    return defaultdict(set)

@dataclass
class RuleSet:
    comes_before: dict[int, set[int]] = field(default_factory=_dict_of_sets)
    """Set of numbers that come _before_ key."""
    comes_after: dict[int, set[int]] = field(default_factory=_dict_of_sets)
    """Set of numbers that come _after_ key."""

    def insert_rule(self, before: int, after: int) -> None:
        self.comes_before[after].add(before)
        self.comes_after[before].add(after)

    def is_ordered(self, numbers: list[int]) -> bool:
        prev = set()
        next = set(numbers)
        for n in numbers:
            next.remove(n)
            if next & self.comes_before[n]:
                return False
            if prev & self.comes_after[n]:
                return False
            prev.add(n)
        return True
    
    def sort_set(self, numbers: set[int]) -> list[int]:
        if len(numbers) <= 1:
            return list(numbers)
        pivot = numbers.pop()
        left_set = self.comes_before[pivot] & numbers
        right_set = self.comes_after[pivot] & numbers
        return self.sort_set(left_set) + [pivot] + self.sort_set(right_set)


def load_input() -> tuple[RuleSet, list[list[int]]]:
    rules = RuleSet()
    updates = []
    with open("input/05.txt") as f:
        for line in f:
            if not line.strip():
                break
            before, after = line.split("|")
            rules.insert_rule(int(before), int(after))
        for line in f:
            updates.append([int(x) for x in line.strip().split(",")])

    return rules, updates


@adventlib.timeit()
def part1() -> int:
    rules, updates = load_input()
    total = 0
    for u in updates:
        if rules.is_ordered(u):
            total += u[len(u) // 2]
    return total


@adventlib.timeit()
def part2() -> int:
    rules, updates = load_input()
    total = 0
    for u in updates:
        sorted = rules.sort_set(set(u))
        if sorted != u:
            total += sorted[len(sorted) // 2]
    return total


if __name__ == "__main__":
    print(f"Part 1: {part1()=}")
    print(f"Part 2: {part2()=}")
