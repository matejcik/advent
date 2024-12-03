import typing as t
from dataclasses import dataclass


def band(min: int, max: int) -> range:
    return range(min, max + 1)


def differences(it: t.Iterator[int]) -> t.Iterator[int]:
    prev = next(it)
    for level in it:
        yield level - prev
        prev = level


def is_in_band(it: t.Iterator[int], band: range) -> bool:
    return all(x in band for x in differences(it))


def skip_nth(it: t.Iterator[int], n: int) -> t.Iterator[int]:
    for i, level in enumerate(it):
        if i != n:
            yield level


@dataclass
class Report:
    levels: list[int]

    BAND_NEGATIVE: t.ClassVar[range] = band(-3, -1)
    BAND_POSITIVE: t.ClassVar[range] = band(1, 3)

    @classmethod
    def parse(cls, line: str) -> t.Self:
        levels = [int(x) for x in line.strip().split()]
        return cls(levels)

    def is_safe(self) -> bool:
        return is_in_band(iter(self.levels), self.BAND_NEGATIVE) or is_in_band(
            iter(self.levels), self.BAND_POSITIVE
        )

    def is_almost_safe(self) -> bool:
        for n in range(len(self.levels)):
            if is_in_band(skip_nth(iter(self.levels), n), self.BAND_NEGATIVE):
                return True
            if is_in_band(skip_nth(iter(self.levels), n), self.BAND_POSITIVE):
                return True
        return False

def main():
    with open("input/02.txt") as f:
        reports = [Report.parse(line) for line in f]

    safe_reports = [r for r in reports if r.is_safe()]
    print(f"Part 1: {len(safe_reports)=}")

    safe_dampened = [r for r in reports if r.is_almost_safe()]
    print(f"Part 2: {len(safe_dampened)=}")


if __name__ == "__main__":
    main()
