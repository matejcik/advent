from __future__ import annotations
from collections import defaultdict

XRANGE = (282, 314)
YRANGE = (-80, -45)

# step 1: find all valid X coordinates
target_x_min, target_x_max = XRANGE
x_steps: dict[int, set[int]] = defaultdict(set)

# the lowest possible X velocity is V such that (V * (V + 1)) / 2 == target_x_max
# but who would bother calulating when we can start at 1 and go up
# the highest possible X velocity is target_x_max -- any higher and we'll overshoot in step 1
for maybe_x in range(1, target_x_max + 1):
    xdist = 0
    xvel = maybe_x
    step = 0
    while xdist <= target_x_max and xvel > 0:
        step += 1
        xdist += xvel
        xvel -= 1
        if target_x_min <= xdist <= target_x_max:
            x_steps[step].add(maybe_x)
    if xvel == 0 and target_x_min <= xdist <= target_x_max:
        # if we're stopped in the x range of the target, we can wait there
        for s in range(step, 1000):
            x_steps[s].add(maybe_x)

target_y_min, target_y_max = YRANGE
y_steps: dict[int, set[int]] = defaultdict(set)
# the lowest possible Y velocity is target_y_min, assuming target_y_min < 0
# any lower and we shoot below the target area in step 1
# the highest possible Y velocity is -target_y_min - 1, which will fly high and dip into
# the target zone one step after going back down
for maybe_y in range(target_y_min, -target_y_min):
    ydist = 0
    yvel = maybe_y
    step = 0
    while ydist >= target_y_min:
        step += 1
        ydist += yvel
        yvel -= 1
        if target_y_min <= ydist <= target_y_max:
            y_steps[step].add(maybe_y)

# consolidate:
max_step = max(max(x_steps.keys()), max(y_steps.keys()))
velocities: set[tuple[int, int]] = set()
for step in range(max_step + 1):
    for x in x_steps[step]:
        for y in y_steps[step]:
            velocities.add((x, y))

# print(f"Part 1: {-target_y_min - 1}")  # this might fail for some inputs (that probably don't exist in the game)
max_y_vel = max(y for _x, y in velocities)
max_y_height = (max_y_vel * (max_y_vel + 1)) // 2
print(f"Part 1: {max_y_height}")
print(f"Part 2: {len(velocities)}")
