from __future__ import annotations

increments = 0

# count the first entry as an increment
prev_value = -1
for line in open("01.txt"):
    n = int(line)
    assert n > 0  # so that prev_value == -1 works the way we want
    if n > prev_value:
        increments += 1
    prev_value = n

print("answer 1:", increments - 1)  # discount the first increment

increments = 0
sliding_window: list[int] = []
for line in open("01.txt"):
    n = int(line)
    prev_sum = sum(sliding_window)
    sliding_window.append(n)
    if len(sliding_window) < 3:
        continue
    sliding_window = sliding_window[-3:]
    current_sum = sum(sliding_window)
    if current_sum > prev_sum:
        increments += 1

print("answer 2:", increments - 1)  # discount the first increment
