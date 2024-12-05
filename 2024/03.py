import re
from pathlib import Path

import adventlib

HERE = Path(__file__).parent

PROGRAM_TXT = HERE / "input" / "03.txt"
PROGRAM = PROGRAM_TXT.read_text()

PROGRAM_RE = re.compile(r"(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\)")


@adventlib.timeit()
def part1_mul_all(program: str) -> int:
    total = 0
    for match in PROGRAM_RE.finditer(program):
        do, dont, mul, a, b = match.groups()
        if mul:
            total += int(a) * int(b)
    return total


@adventlib.timeit()
def part2_mul_some(program: str) -> int:
    total = 0
    enabled = True
    for match in PROGRAM_RE.finditer(program):
        do, dont, mul, a, b = match.groups()
        if do:
            enabled = True
        elif dont:
            enabled = False
        elif mul and enabled:
            total += int(a) * int(b)
    return total


if __name__ == "__main__":
    print(f"Part1: {part1_mul_all(PROGRAM)=}")
    print(f"Part2: {part2_mul_some(PROGRAM)=}")
