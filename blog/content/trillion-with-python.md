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
You're given a two-part puzzle every day. Usually, day 1 is the easier version and day 2
is much harder variant reusing the same input.

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

I learned about [Matt Keeter]'s solution from Reddit. Unlike the standard approach, this
solution is _general_: it can solve any program, not just the one from the puzzle.

[Matt Keeter]: https://www.mattkeeter.com/blog/2021-12-27-brute/

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
2. If it is anything else than `inp`, execute it on all currently tracked states and
   goto 1.  
   Process `inp` instruction by continuing:
3. Deduplicate states: identify states that are the same and collapse them into one,
   keeping the minimum and the maximum that got us there.
4. Make 9 copies of each state. Set the input digit to be different in each copy.
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
        self.states: list[ALUState] = [ALUState()]

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

Off the screen, I'll add some timing information... and

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
