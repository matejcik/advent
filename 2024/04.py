import numpy as np

NEEDLE = np.array(list(b"XMAS"), dtype=np.uint8)
N = len(NEEDLE)
NEEDLE_BACKWARDS = NEEDLE[::-1]

def load_input() -> np.ndarray:
    lines = []
    with open("input/04.txt") as f:
        for line in f:
            lines.append([ord(x) for x in line.strip()])
    return np.array(lines, dtype=np.uint8)


def testneedle(haystack: np.array) -> bool:
    return np.array_equal(haystack, NEEDLE) or np.array_equal(haystack, NEEDLE_BACKWARDS)


def part1_search_xmas(data: np.ndarray) -> int:
    x, y = data.shape
    total = 0
    for i in range(x):
        for j in range(y):
            if testneedle(data[i, j:j + N]):
                total += 1
            if testneedle(data[i:i + N, j].transpose()):
                total += 1
            diag = data[i:i + N, j:j + N]
            if testneedle(diag.diagonal()):
                total += 1
            if testneedle(np.fliplr(diag).diagonal()):
                total += 1

    return total

def part2_search_x_mas(data: np.ndarray) -> int:
    x, y = data.shape
    A = ord("A")
    M = ord("M")
    S = ord("S")
    total = 0
    for i in range(x - 2):
        for j in range(y - 2):
            testregion = data[i:i + 3, j:j + 3]
            assert testregion.shape == (3, 3)
            if testregion[1,1] != A:
                continue
            diag_a = testregion.diagonal()
            diag_b = np.fliplr(testregion).diagonal()
            if M in diag_a and S in diag_a and M in diag_b and S in diag_b:
                total += 1
    return total


if __name__ == "__main__":
    data = load_input()
    print(f"Part 1: {part1_search_xmas(data)=}")
    print(f"Part 2: {part2_search_x_mas(data)=}")
