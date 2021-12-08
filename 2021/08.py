from typing import Iterable
from collections import defaultdict

def analyze_digits(entries: Iterable[frozenset[str]]) -> list[frozenset[str]]:
    digits: list[frozenset[str] | None] = [None] * 10

    lengths: dict[int, list[frozenset[str]]] = defaultdict(list)
    for entry in entries:
        lengths[len(entry)].append(entry)

    # digit 1 has length 2
    assert len(lengths[2]) == 1
    digits[1] = lengths[2][0]
    # digit 4 has length 4
    assert len(lengths[4]) == 1
    four = digits[4] = lengths[4][0]
    # digit 7 has length 3
    assert len(lengths[3]) == 1
    seven = digits[7] = lengths[3][0]
    # digit 8 has length 7
    assert len(lengths[7]) == 1
    digits[8] = lengths[7][0]

    # discriminate between 6, 9, 0 (length 6)
    sixes = lengths[6]
    assert len(sixes) == 3
    for six in sixes:
        if len(six - seven) == 4:
            # it's a 6
            digits[6] = six
        elif len(six - four) == 2:
            # it's a 9
            digits[9] = six
        else:
            # it's a 0
            digits[0] = six

    # discriminate between 5, 3, 2 (length 5)
    fives = lengths[5]
    assert len(fives) == 3
    for five in fives:
        if len(five - seven) == 2:
            # it's a 3
            digits[3] = five
        elif len(five - four) == 2:
            # it's a 5
            digits[5] = five
        else:
            # it's a 2
            digits[2] = five

    assert all(d is not None for d in digits)
    return digits  # type: ignore


def digit_to_number(digits: list[frozenset[str]], digit: frozenset[str]) -> int:
    return digits.index(digit)


total_ones_fours_sevens_eights = 0
total_decimal_outputs = 0


with open("08.txt") as f:
    for line in f:
        line = line.strip()
        digit_part, output_part = line.split(" | ")
        entries = [frozenset(e) for e in digit_part.split()]
        outputs = [frozenset(e) for e in output_part.split()]
        digits = analyze_digits(entries)
        numbers = [digit_to_number(digits, d) for d in outputs]
        total_ones_fours_sevens_eights += sum(n in (1, 4, 7, 8) for n in numbers)

        decimal_number = int("".join(str(n) for n in numbers))
        total_decimal_outputs += decimal_number

print(f"Part 1: {total_ones_fours_sevens_eights}")
print(f"Part 2: {total_decimal_outputs}")
