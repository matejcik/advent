from typing import Tuple

Pixel = Tuple[int, int]

GRID = {}

with open('09.txt') as f:
    lines = f.readlines()
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            GRID[x, y] = int(char)

def neighbors(pixel: Pixel):
    px, py = pixel
    for ox, oy in ((0, -1), (1, 0), (0, 1), (-1, 0)):
        dest = px + ox, py + oy
        if dest in GRID:
            yield dest


# part 1
total = 0
for pixel in GRID:
    if all(GRID[pixel] < GRID[n] for n in neighbors(pixel)):
        total += 1 + GRID[pixel]

print(f"Part 1: {total}")

# part 2:
BASINS: list[set[Pixel]] = []
for pixel in GRID:
    if any(pixel in basin for basin in BASINS):
        continue

    if GRID[pixel] >= 9:
        continue

    basin = set()
    queue = [pixel]
    while queue:
        pixel = queue.pop(0)
        basin.add(pixel)
        for n in neighbors(pixel):
            if GRID[n] >= 9:
                continue
            if n in basin or n in queue:
                continue
            queue.append(n)

    BASINS.append(basin)

BASINS.sort(key=len, reverse=True)
print(f"Part 2: {len(BASINS[0]) * len(BASINS[1]) * len(BASINS[2])}")
