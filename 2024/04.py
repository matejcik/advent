import numpy as np

import adventlib

NEEDLE = b"XMAS"
N = len(NEEDLE)
NEEDLE_BACKWARDS = NEEDLE[::-1]


def load_input() -> np.ndarray:
    lines = []
    with open("input/04.txt") as f:
        for line in f:
            lines.append([ord(x) for x in line.strip()])
    return np.array(lines, dtype=np.uint8)


def testneedle(haystack: np.array) -> bool:
    b = haystack.tobytes()
    return b == NEEDLE or b == NEEDLE_BACKWARDS


def diagonal_shift(data: np.ndarray) -> np.ndarray:
    newdata = data.copy()
    x, y = data.shape
    for i in range(y):
        newdata[i, :] = np.concatenate((data[i, i:], data[i, :i]))
    return newdata


@adventlib.timeit(2)
def part1_search_xmas(data: np.ndarray) -> int:
    transposed = data.transpose()
    diag_a = diagonal_shift(data)
    diag_b = diagonal_shift(np.fliplr(data))

    def find_in(data: np.ndarray) -> int:
        total = 0
        x, y = data.shape
        for j in range(y):
            for i in range(x):
                if testneedle(data[i : i + N, j]):
                    total += 1
        return total

    return find_in(data) + find_in(transposed) + find_in(diag_a) + find_in(diag_b)


@adventlib.timeit()
def part2_search_x_mas(data: np.ndarray) -> int:
    x, y = data.shape
    A = ord("A")
    M = ord("M")
    S = ord("S")
    total = 0
    for i in range(1, x - 1):
        for j in range(1, y - 1):
            if data[i, j] != A:
                continue
            diag_a = data[i - 1, j - 1], data[i + 1, j + 1]
            diag_b = data[i - 1, j + 1], data[i + 1, j - 1]
            if M in diag_a and M in diag_b and S in diag_a and S in diag_b:
                total += 1
    return total


if __name__ == "__main__":
    data = load_input()
    print(f"Part 1: {part1_search_xmas(data)=}")
    print(f"Part 2: {part2_search_x_mas(data)=}")
