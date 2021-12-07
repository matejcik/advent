from collections import Counter

with open("07.txt") as f:
    DATA = [int(n) for n in f.read().split(",")]

# part 1
DATA.sort()

median = DATA[len(DATA) // 2]
sum_of_distances = sum(abs(n - median) for n in DATA)
print(f"Part 1: {sum_of_distances}")

# part 2
CRABS = Counter(DATA)
leftmost = DATA[0]
rightmost = DATA[-1]

min_total_fuel = 0xffff_ffff

for candidate in range(leftmost, rightmost + 1):
    total_fuel = 0
    for crab in range(leftmost, rightmost + 1):
        dist = abs(candidate - crab)
        fuel_per_crab = (dist * (dist + 1)) // 2
        total_fuel += CRABS[crab] * fuel_per_crab
    min_total_fuel = min(min_total_fuel, total_fuel)

print(f"Part 2: {min_total_fuel}")
