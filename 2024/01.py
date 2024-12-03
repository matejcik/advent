# load input
numbers = []
with open("input/01.txt") as f:
    for line in f:
        x, y = line.strip().split(maxsplit=1)
        numbers.append((int(x), int(y)))

# that ol' trick to transpose a list
left_col, right_col = zip(*numbers)
left_col = list(left_col)
right_col = list(right_col)

### part 1 ###

# get the smallest and largest values aligned
left_col.sort()
right_col.sort()

total_distance = sum(abs(x - y) for x, y in zip(left_col, right_col))

print(f"Part 1: {total_distance=}")

### part 2 ###

import collections

right_num_counter = collections.Counter(right_col)
left_multiples = [i * right_num_counter[i] for i in left_col]

similarity_score = sum(left_multiples)

print(f"Part 2: {similarity_score=}")
