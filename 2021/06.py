NEWBORN_SPAWN_TIME = 8
REPEAT_SPAWN_TIME = 6


def advance_population(population: list[int]) -> list[int]:
    """Advance lanternfish population by one day.

    For every `i`, the value `population[i]` is the number of lanternfish whose
    reproduction timer is at that value.

    Once a fish's timer hits zero, the next day it moves to `REPEAT_SPAWN_TIME` bucket,
    and produces an offspring in `NEWBORN_SPAWN_TIME` bucket.
    """
    # advance all timers by one, push new one at end
    next_day = population[1:] + [0]
    # make newborns
    next_day[NEWBORN_SPAWN_TIME] += population[0]
    # restart those who gave birth
    next_day[REPEAT_SPAWN_TIME] += population[0]
    return next_day

# set up population given by timers in the input file
population = [0] * (max(NEWBORN_SPAWN_TIME, REPEAT_SPAWN_TIME) + 1)
with open("06.txt") as f:
    line = f.readline()
    for fish in line.split(","):
        population[int(fish)] += 1

for _ in range(80):
    population = advance_population(population)

print(f"Part 1: {sum(population)} total fish after 80 days - {population}")

for _ in range(256 - 80):
    population = advance_population(population)

print(f"Part 1: {sum(population)} total fish after 256 days - {population}")
