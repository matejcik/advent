depth = 0
horiz = 0

for line in open("02.txt"):
    match line.strip().split():
        case ["forward", n]:
            horiz += int(n)
        case ["up", n]:
            depth -= int(n)
        case ["down", n]:
            depth += int(n)
        case _:
            raise ValueError(f"Unknown command: {line}")

print(f"Part 1: {horiz * depth}")

depth = 0
horiz = 0
aim = 0

for line in open("02.txt"):
    cmd, nstr = line.strip().split(maxsplit=1)
    n = int(nstr)
    match cmd:
        case "forward":
            horiz += n
            depth += aim * n
        case "up":
            aim -= n
        case "down":
            aim += n

print(f"Part 2: {horiz * depth}")
