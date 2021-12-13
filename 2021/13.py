class Paper:
    def __init__(self) -> None:
        self.dots: set[tuple[int, int]] = set()

    def paint(self, x: int, y: int) -> None:
        self.dots.add((x, y))

    @staticmethod
    def fold_coord(value: int, fold: int) -> int:
        return fold - abs(value - fold)

    def fold_x(self, fold: int) -> None:
        self.dots = {(self.fold_coord(x, fold), y) for (x, y) in self.dots}

    def fold_y(self, fold: int) -> None:
        self.dots = {(x, self.fold_coord(y, fold)) for (x, y) in self.dots}

    def draw(self) -> str:
        max_x = max(x for (x, _) in self.dots)
        max_y = max(y for (_, y) in self.dots)
        return "\n".join(
            "".join("#" if (x, y) in self.dots else "." for x in range(max_x + 1))
            for y in range(max_y + 1)
        )


paper = Paper()
with open("13.txt") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        if line.startswith("fold"):
            _fold, _along, val = line.split()
            axis, num = val.split("=")
            if axis == "x":
                paper.fold_x(int(num))
            else:
                paper.fold_y(int(num))
            print(f"Folded along {axis}={num}, {len(paper.dots)} dots remain")

        else:
            x, y = line.split(",")
            paper.paint(int(x), int(y))

print("Final image:")
print(paper.draw())
