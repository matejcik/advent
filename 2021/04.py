from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class Board:
    numbers: list[list[int]]
    rows: list[set[int]] = field(init=False)
    cols: list[set[int]] = field(init=False)
    score: int | None = None

    @classmethod
    def from_block(cls, block: str) -> Board:
        lines = block.splitlines()
        numbers = [[int(n) for n in line.split()] for line in lines]

        board = cls(numbers)
        board.reset()
        return board

    def reset(self) -> None:
        self.score = None
        self.rows = [set(row) for row in self.numbers]
        self.cols = [set(col) for col in zip(*self.numbers)]


    def is_done(self) -> bool:
        return self.score is not None

    def _calculate_score(self, number: int) -> int:
        return sum(n for row in self.rows for n in row) * number

    def play(self, number: int) -> None:
        if self.is_done():
            return

        for line in self.rows + self.cols:
            line.discard(number)

        if any(not line for line in self.rows + self.cols):
            self.score = self._calculate_score(number)


with open("04.txt") as f:
    INPUT_GROUPS = f.read().split("\n\n")

NUMBERS = [int(n) for n in INPUT_GROUPS[0].split(",")]
BOARDS = [Board.from_block(block) for block in INPUT_GROUPS[1:]]


# part 1
for number in NUMBERS:
    for i, board in enumerate(BOARDS):
        if board.is_done():
            continue
        board.play(number)
        if board.is_done():
            print(f"Board {i} wins with score {board.score}")
