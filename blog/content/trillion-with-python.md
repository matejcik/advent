Title: 22 trillion parameters in Python
Date: 2022-01-01 11:00
Category: Python
Slug: trillion-py
Summary: Can we brute-force a 14-digit number in Python and 16 GB of RAM?

This year, I decided to be lazy and do [Advent of Code] in Python. I am very comfortable
in Python and didn't want to fight the language as well as the puzzles for once. For
that reason, it's the first time that I actually completed almost all the puzzles.

[Advent of Code]: https://adventofcode.com/

I originally used a [pre-trained neural net] to solve [Day 24]. By the time I understood
how the problem works, there was no programming left to do. But then I found out about
a [clever general solution] and decided to try to implement it myself.

[Day 24]: https://adventofcode.com/2021/day/24
[pre-trained neural net]: https://xkcd.com/2173/
[clever general solution]: https://www.mattkeeter.com/blog/2021-12-27-brute/

**If you don't know about Advent of Code...**

[Advent of Code] is a programming game that runs every year for the 25 days of Advent.
You're given a two-part puzzle every day. Usually, part 1 is the easier version and part
2 is a much harder variant reusing the same input.

**If you don't know about 2021 day 24...**

[Day 24] defines a very simple computer, or **ALU**. It has four integer registers and a
bunch of basic instructions: `add`, `mul`, `div`, `mod`, `eql`. There are no jumps, no
loops, so it's nothing more than a calculator. The instruction `inp` reads one digit of
a 14-digit input.

The puzzle is a [program] for this computer that does _some calculation_. This
calculation sometimes ends with a zero result. The goal is to find the smallest and
largest 14-digit number that causes the result to be zero.

[program]: https://github.com/matejcik/advent/blob/main/2021/24.txt

Trying the calculation for all possible 14-digit numbers would take a lot of time --
even with digit 0 disallowed, there are 9^14 = 22 trillion possibilities. The standard
solution is to read the puzzle program, understand what it calculates, and use this
knowledge to reduce the search space.

**If you don't know about the brute-force solution...**

I learned about [Matt Keeter's solution] from Reddit. Unlike the standard approach, this
solution is _general_: it can solve any program, not just the one from the puzzle.

[Matt Keeter's solution]: https://www.mattkeeter.com/blog/2021-12-27-brute/

The basic idea is, instead of trying the full calculation 22 trillion times, we can keep
track of what the _possible states_ of the computer are at any given step.

At start, there is only one possible state: all registers are zero. The arithmetic
instructions only convert the state into another state. The only way to create new
states is via the `inp` instruction, which loads a new digit.

So for example, this one-line program:
```
inp x
```
will result in nine different possible states:
```
x: 1, y: 0, z: 0, w: 0
x: 2, y: 0, z: 0, w: 0
x: 3, y: 0, z: 0, w: 0
x: 4, y: 0, z: 0, w: 0
x: 5, y: 0, z: 0, w: 0
x: 6, y: 0, z: 0, w: 0
x: 7, y: 0, z: 0, w: 0
x: 8, y: 0, z: 0, w: 0
x: 9, y: 0, z: 0, w: 0
```

Another thing to notice is that some instructions can _reduce_ the number of possible
states. In the example above, the instruction `mul x 0` will collapse the nine different
states back into one: whatever was previously in `x`, there is now a zero.

If the puzzle program behaves at least somewhat nicely, we will never reach 22 trillion
states. Hopefully, the total number of states is going to be manageable.

To get the puzzle solution, we just need to keep track of what was the largest and
smallest number that got us to some state; one of those will be the answer.


## The algorithm

Here is a high-level outline of how the solution will work:

1. Read the next instruction.
2. If it is anything else than `inp`, execute it on all currently tracked states.
3. If it is `inp`:
    1. Deduplicate states: identify states that are the same and collapse them into one,
    keeping the minimum and the maximum that got us there.
    2. Make 9 copies of each state. Set the input digit to be different in each copy.
5. Repeat until end of program.


## Getting ready

Let's set up the basics.

We need a data structure to represent one state of the computer:
```python
REGISTERS = "xyzw"

class ALUState:
    def __init__(self):
        self.regs = {reg: 0 for reg in REGISTERS}
        self.min = 0
        self.max = 0
```

We need an instruction table. We're going to be somewhat smart from the start: instead
of having a bunch of `if/elif` branches for every instruction, we will save a _function_
that performs the operation we want. A lot of those live in the built-in `operator`
module.

The function `operator.eq` is a little interesting: it takes two integers and returns a
_boolean_, but we want 0 or 1 as the result. But it just so happens that in Python,
`True == 1` and `False == 0`.
```python
import operator

INSTRUCTIONS = {
    "add": operator.add,
    "mul": operator.mul,
    "div": operator.ifloordiv,
    "mod": operator.mod,
    "eql": operator.eq,
}
```

Now, let's implement a "[non-deterministic] ALU" -- a computer that, essentially, is in
all possible states at once.

[non-deterministic]: https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton
```python
class NonDeterministicALU:
    def __init__(self):
        # at start, we only have one all-zero state
        self.states = [ALUState()]

    def run(self, program):
        """Run whole program."""
        for line in program.splitlines():
            self.execute(line)

    def find_puzzle_answer(self):
        """Find the smallest and largest number that got us
        to a state with zero."""
        zero_states = (state for state in self.states
                       if state.regs["z"] == 0)
        result_min = 10 ** 15  # all 14-digit inputs are smaller
        result_max = 0         # all 14-digit inputs are larger
        for state in zero_states:
            # find minimum and maximum in one pass
            result_min = min(result_min, state.min)
            result_max = max(result_max, state.max)
        return result_min, result_max

    def execute(self, line):
        """Execute one line of the program."""
        if line.startswith("inp"):
            # inp is handled separately
            self.execute_input(line)
            return

        # split into instruction, destination register and source
        instr, dest, src = line.split()
        instr_func = INSTRUCTIONS[instr]
        if src in REGISTERS:
            # src can either be one of x, y, z, w, or an integer
            # literal value. We expect a lot of states, so we don't
            # want to do "if src in REGISTERS" every time. So we
            # do it ahead of time and call the appropriate function.
            self.execute_register(instr_func, dest, src)
        else:
            self.execute_literal(instr_func, dest, int(src))

    def execute_register(self, instr_func, dest, src):
        """Execute an instruction that takes a register as source."""
        for state in self.states:
            # call the `instr_func` on two registers
            # assign the result to dest
            state.regs[dest] = instr_func(state.regs[dest],
                                          state.regs[src])

    def execute_literal(self, instr_func, dest, val):
        """Execute an instruction that takes a literal value."""
        for state in self.states:
            # call the `instr_func` on a register and value
            state.regs[dest] = instr_func(state.regs[dest], val)
```

Let's look at the `execute_input()` method separately. By itself, it is very simple:
call the deduplication step, then call the expansion step.
```python
    def execute_input(self, line):
        """Execute the `inp` instruction."""
        instr, dest = line.split()
        assert instr == "inp"
        assert dest in REGISTERS
        # first, reduce the number of states by deduplication
        self.deduplicate()
        # second, make a copy of all states and set the input digit
        self.expand_states_into(dest)
```

We will deduplicate via a dict. The key is a tuple of register values, and the value
is the first such state that we found.
```python
    def deduplicate(self):
        known_states = {}
        new_states = []
        for state in self.states:
            key = tuple(state.regs.values())
            if key in known_states:
                self.update_minmax(known_states[key], state)
            else:
                known_states[key] = state
                new_states.append(state)
        self.states = new_states

    @staticmethod
    def update_minmax(state1, state2):
        state1.min = min(state1.min, state2.min)
        state1.max = max(state1.max, state2.max)
```

To expand the states, we will only make 8 copies of each, and modify the last one
in-place. This is also why we can't use `for state in self.states` directly: we will be
adding states in the for-loop, so we would never exit.
```python
    def expand_states_into(self, dest):
        for i in range(len(self.states)):
            for digit in range(2, 10):
                # digits 2 to 9
                new_state = deepcopy(self.states[i])
                self.set_digit(new_state, dest, digit)
                self.states.append(new_state)
            # digit 1
            self.set_digit(self.states[i], dest, 1)

    @staticmethod
    def set_digit(state, dest, digit):
        state.regs[dest] = digit
        state.min = state.min * 10 + digit
        state.max = state.max * 10 + digit
```

The `set_digit()` step increases the length of the current minimums and maximums. If we
already executed three `inp` instructions, our minimums and maximum are 3-digit numbers.
Another `inp` must increase it to a 4-digit one, by adding the new digit to the right.

This also means that we _first_ need to make the copies and _then_ modify the original
-- otherwise, the copies would get two more digits instead of one.

Now the runner itself:
```python
from pathlib import Path

INPUT_FILE = Path("input.txt")
alu = NonDeterministicALU()
alu.run(INPUT_FILE.read_text())

result_min, result_max = alu.find_puzzle_answer()
print("Minimum:", result_min)
print("Maximum:", result_max)
```

Off the screen, I'll add some timing information ([complete source](trillion/trillion-naive.py)) ... and


## Let's run it!


```
% python trillion.py
=== digit 1 ===
deduplication: 0.000 s
expansion: 0.000 s
continuing with 9 states
=== digit 2 ===
deduplication: 0.000 s
expansion: 0.001 s
continuing with 81 states
=== digit 3 ===
deduplication: 0.000 s
expansion: 0.011 s
continuing with 729 states
=== digit 4 ===
deduplication: 0.001 s
expansion: 0.081 s
continuing with 6561 states
=== digit 5 ===
deduplication: 0.008 s
expansion: 0.102 s
continuing with 8748 states
=== digit 6 ===
deduplication: 0.006 s
expansion: 0.732 s
continuing with 65610 states
```
Cool! Going nice!
```
=== digit 7 ===
deduplication: 0.061 s
expansion: 7.238 s
continuing with 590490 states
=== digit 8 ===
deduplication: 0.555 s
expansion: 8.526 s
continuing with 721710 states
=== digit 9 ===
deduplication: 0.594 s
expansion: 77.458 s
continuing with 5878656 states
```
...okay...
```
=== digit 10 ===
deduplication: 5.064 s
[1]    220471 killed     python trillion.py
```
![That escalated quickly](images/escalated.jpg)

What happened is I ran out of RAM. My laptop only has 16 GB, plus 1 GB swap. Even after
shutting down the browser and vscode, it was not enough.

I briefly toyed with increasing swap space, but it turns out that as soon as the
calculation overflows to swap, it grinds to a halt. It might not finish in time for the
next Advent.

We need to reduce RAM usage somehow.

## Reducing RAM usage

First idea: the ALU state is just six numbers. We don't need a dict and a class to
represent six numbers, we could use a simple list.

Each state will be represented by a list of six numbers. Indices 0 to 3 represent
registers `x` to `w`. Index 4 is minimum and index 5 is maximum.

```python
# instead of
state.regs[dest] = value
state.min = some_minimum
# do
dest_reg = REGISTERS.index[dest]
state[dest_reg] = value
state[STATE_MIN] = some_minimum
```

Not much more interesting beyond that, you can [see the code](trillion/trillion-with-list.py)
for yourself.

Trying again:
```
...
=== digit 7 ===
deduplication: 0.043 s
expansion: 0.553 s
continuing with 590490 states
=== digit 8 ===
deduplication: 0.472 s
expansion: 0.745 s
continuing with 721710 states
=== digit 9 ===
deduplication: 0.480 s
expansion: 6.421 s
continuing with 5878656 states
=== digit 10 ===
deduplication: 4.515 s
expansion: 58.542 s
continuing with 52907904 states
```
Better!
```
=== digit 11 ===
[1]    225074 killed     python trillion-with-list.py
```
Killed before deduplication finished! **Oh no!** The list of states is now so large that
we can't even construct the deduplicating dict.

Or maybe the list of `new_states` is the problem... Let's try a tweak.
```python
    def deduplicate(self):
        known_states = {}
        cursor = 0
        for current in range(len(self.states)):
            state = self.states[current]
            key = tuple(state[:4])
            if key in known_states:
                self.update_minmax(known_states[key], state)
            else:
                known_states[key] = state
                self.states[cursor] = state
                cursor += 1
        del self.states[cursor:]
```
Instead of creating a new list of states, we will reuse the existing one. `cursor`
points to the first spot that we can replace. At start, the "replace" will be a no-op,
we will be replacing the state with itself. In all subsequent cases, we are replacing
something that was either (a) already moved to the front, or (b) ignored because it is a
duplicate state.

At the end, we delete the rest of the list, so that only the copied-over states remain.

[Trying](trillion/trillion-list-dedup.py)...
```
=== digit 11 ===
deduplication: 46.264 s
[1]    226107 killed     python trillion-list-dedup.py
```
Well look at that, it helped!

A little.

First of all, the deduplication now takes 46 seconds. Second, we are at the limit
anyway, the expansion step won't finish.

## What's smaller than a list?

A list of six numbers _should_ be small. But it really isn't. For one, Python's lists
can store any object. This means that the list actually stores _pointers_. For every
item, you need to store 8-byte pointer, plus the item object itself.

Python's numbers are also objects. Even if a 64-bit number only takes 64 bits (8 bytes)
of memory, there is at least 8 more bytes indicating object type. There is some sort of
optimization for "small numbers", but we can't be sure that it applies for our usecase.

(This is true for CPython, the "official" interpreter from [python.org]. It just might
turn out that using, e.g., [PyPy], our RAM requirements would be lower from the get-go
and we could just stop here.)

[python.org]: https://www.python.org/
[PyPy]: https://pypy.org/

Enter [`array`](https://docs.python.org/3/library/array.html). It's an "efficient array
of numeric values". Which is exactly what we need here.

Let's try it. The change is basically two lines:
```python
from array import array

class NonDeterministicALU:
    def __init__(self):
        # instead of:
        # self.states = [[0] * 6]
        # use:
        self.states = [array('q', [0] * 6)]
```
Using `'q'` for the data type, which is a "signed long long", or, in human speak, a
64-bit integer with support for negative values. We don't know how big the numbers get
during the computation, but we do know that negative values are allowed, so I'm picking
the largest number type that I have. It takes 8 bytes of memory, but can never grow
beyond that.

Running [the program](https://github.com/matejcik/advent/blob/main/2021/24-array.py) again...
```
=== digit 10 ===
deduplication: 5.544 s
expansion: 37.640 s
continuing with 52907904 states
```
Twice as fast as before, half as much RAM consumed. We're now at 6 GB. The previous
version consumed 13 GB for digit 10.
```
=== digit 11 ===
deduplication: 58.994 s
expansion: 65.493 s
continuing with 88179840 states
=== digit 12 ===
deduplication: 98.897 s
expansion: 71.800 s
continuing with 101702790 states
=== digit 13 ===
deduplication: 107.779 s
expansion: 59.891 s
continuing with 95866416 states
=== digit 14 ===
deduplication: 102.136 s
expansion: 68.294 s
continuing with 107811162 states
Computation finished in 1623.035 s
Minimum: 41171183141291
Maximum: 91398299697996
```
**...and done.**

RAM usage peaked around 14 GB. Fortunately, it turns out that the number of states stops
growing around digit 11.

Total comuputation time is **1 623 seconds**. Not awesome, especially compared to the
[Rust solution] which can do the same in 30 seconds. But pretty good, all things
considered!

[Rust solution]: https://www.mattkeeter.com/blog/2021-12-27-brute/


## Can we do better?

Not with stock Python. I don't know how, anyway; I could micro-optimize some parts, but
it would shave off maybe a couple seconds.

But there's no need to stop there. Instead, let's look at [Pandas].

[Pandas]: https://pandas.pydata.org/

Pandas is a data science library, designed to efficiently work with large data sets.
Where "efficiently" translates to pretty much **like magic**. In the first part of the
article, we were thinking about doing things in-place, not needlessly reallocating
memory, looking for the most space-efficient data structures.

With Pandas, I can just write what I mean and It. Just. Works. Zero hassle.

First, we create a _dataframe_.

```python
class NonDeterministicALU:
    def __init__(self):
        self.states = pd.DataFrame({
            "x": [0], "y": [0], "z": [0], "w": [0],
            "min": [0], "max": [0]})
```

The syntax means that we have columns `"x", "y", "z", "w", "min", "max"`, and one row in
which all columns are zero. (There are as many rows as the longest list of values passed
in. If the lists are not the same length, the columns are padded by `None`.)

Finding the minimums and maximums for the puzzle answer is **super easy**:
```python
    def find_puzzle_answer(self):
        zero_states = self.states[self.states["z"] == 0]
        result_min = zero_states["min"].min()
        result_max = zero_states["max"].max()
        return result_min, result_max
```
The first line selects only those states where `z == 0`. The other lines take minimum and maximum from the respective columns.

```python
    def execute(self, line):
        if line.startswith("inp"):
            self.execute_input(line)
            return

        # split into instruction, destination register and source
        instr, dest, src = line.split()
        instr_func = INSTRUCTIONS[instr]
        if src in REGISTERS:
            src_col = self.states[src]
            self.states[dest] = instr_func(self.states[dest], src_col)
        else:
            self.states[dest] = instr_func(self.states[dest], int(src))
```

`self.states[dest]` is the destination column `x`, `y` `z` or `w`. There is no iterating
over rows. I'm calling a function (like `pandas.Series.add`) **on the whole column at
once.**

```python
    def expand_states_into(self, dest):
        new_states = []
        for digit in range(1, 10):
            states = self.states.copy()
            states[dest] = digit
            states["min"] = states["min"] * 10 + digit
            states["max"] = states["max"] * 10 + digit
            new_states.append(states)
        self.states = pd.concat(new_states)
        self.states.reset_index(drop=True, inplace=True)
```

For every digit, I take a copy of the whole state and set the right digit on the copy.
Then I just concatenate all the copies together and save the new state. Notice how,
again, we can set the whole column at once.

(The `reset_index()` bit at the end is for bookkeeping: Pandas keeps an "index" for
every row, and weird things happen when you concatenate two datasets that have the same
indices. I am not using the index anywhere, but calling `reset_index()` like this has
avoided me some problems.)

Finally, the most difficult bit, deduplication:
```python
    def deduplicate(self):
        groups = self.states.groupby(["x", "y", "z", "w"], sort=False)
        self.states = groups.aggregate(
            {"x": "first", "y": "first", "z": "first", "w": "first",
             "min": "min", "max": "max"})
        self.states.reset_index(drop=True, inplace=True)
```

`groupby()` works like SQL `GROUP BY` clause. The result is the data grouped by the
selected columns -- i.e., all rows that have the same `x`, `y`, `z` and `w` will be in
the same group.

By default, Pandas will also sort the result, but we can save some time by not doing
that.

The `aggregate()` call will convert each group into a single row -- by taking the
_first_ value for the `x`, `y`, `z` and `w` columns (they're all identical so it doesn't
matter which we pick), the _minimum_ of `min`s, and the _maximum_ of `max`s.

Finally, I need to call `reset_index()`, otherwise the index value is some sort of
identifier of the original group, which really messes things up.

**That's it** ([complete source]). No messing with dictionaries. I didn't need to
iterate over `self.states` at any point. Every operation can be done via a shorthand.

[complete source]: https://github.com/matejcik/advent/blob/main/2021/24.py

How's the performance?

```
=== digit 10 ===
deduplication: 6.851 s
expansion: 1.628 s
continuing with 52907904 states
=== digit 11 ===
deduplication: 21.151 s
expansion: 2.834 s
continuing with 88179840 states
=== digit 12 ===
deduplication: 31.396 s
expansion: 3.462 s
continuing with 101702790 states
=== digit 13 ===
deduplication: 33.273 s
expansion: 2.884 s
continuing with 95866416 states
=== digit 14 ===
deduplication: 34.208 s
expansion: 3.966 s
continuing with 107811162 states
Computation finished in 174.202 s
```

**174 seconds** total, almost 10x faster than before. RAM usage peaks around 9 GB,
meaning this fits comfortably on my laptop while the browser and vscode is running.

Also, _only_ 10x slower than the best Rust solution before codegen. Considering that
this is Python we're talking about, not bad at all.


## Conclusion

It is possible to do this in pure Python, with a little trickery. But pure Python really
isn't built to handle large datasets. The overhead per item is too large for that.

I took the opportunity to learn a bit of Pandas, and it seems that it is the right tool
for this particular job. I will keep it in mind for future Advent of Code puzzles
:)
