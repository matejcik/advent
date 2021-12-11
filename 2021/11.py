from dataclasses import dataclass
from typing import Tuple, Iterable

Pixel = Tuple[int, int]


@dataclass
class Playground:
    width: int
    height: int
    data: dict[Pixel, int]

    @classmethod
    def from_file(cls, filename: str) -> "Playground":
        with open(filename) as f:
            lines = f.readlines()
        data = {}
        for y, line in enumerate(lines):
            for x, char in enumerate(line.strip()):
                data[(x, y)] = int(char)

        return cls(len(lines[0].strip()), len(lines), data)

    def _neighbors(self, pixel: Pixel) -> Iterable[Pixel]:
        x, y = pixel
        offsets = (-1, 0, 1)
        for oy in offsets:
            for ox in offsets:
                if ox == oy == 0:
                    continue
                neighbor = x + ox, y + oy
                if neighbor not in self.data:
                    continue
                yield neighbor

    def _all_pixels(self) -> Iterable[Pixel]:
        for y in range(self.height):
            for x in range(self.width):
                yield x, y

    def dump(self) -> None:
        for y in range(self.height):
            for x in range(self.width):
                print(self.data[x, y], end="")
            print()

    def _flash(self, pixel: Pixel) -> int:
        flashes = 0
        queue = [pixel]
        while queue:
            p = queue.pop()
            if self.data[p] == -1:
                continue
            flashes += 1
            self.data[p] = -1
            for n in self._neighbors(p):
                if self.data[n] >= 9:
                    queue.append(n)
                elif self.data[n] > -1:
                    self.data[n] += 1

        return flashes

    def step(self) -> int:
        total_flashes = 0
        for p in self._all_pixels():
            if self.data[p] >= 9:
                total_flashes += self._flash(p)
            elif self.data[p] > -1:
                self.data[p] += 1

        for p in self._all_pixels():
            if self.data[p] == -1:
                self.data[p] = 0

        return total_flashes


PLAYGROUND = Playground.from_file("11.txt")

# part 1
total_flashes = 0
for i in range(100):
    total_flashes += PLAYGROUND.step()

print(f"Part 1: {total_flashes}")

# part 2
PLAYGROUND = Playground.from_file("11.txt")
steps = 0
while True:
    steps += 1
    flashes = PLAYGROUND.step()
    if flashes == PLAYGROUND.width * PLAYGROUND.height:
        print(f"Part 2: {steps}")
        break
