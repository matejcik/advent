import pandas as pd

from contexttimer import Timer

REGISTERS = "xyzw"
REG_SELECTOR = list(REGISTERS)

INSTRUCTIONS = {
    "add": pd.Series.add,
    "mul": pd.Series.mul,
    "div": pd.Series.floordiv,
    "mod": pd.Series.mod,
    "eql": pd.Series.eq,
}


class NonDeterministicALU:

    def __init__(self):
        self.states = pd.DataFrame({
            "x": [0], "y": [0], "z": [0], "w": [0],
            "min": [0], "max": [0]})
        self.digit = 0

    def run(self, program):
        """Run whole program."""
        for line in program.splitlines():
            self.execute(line)

    def find_puzzle_answer(self):
        """Find the smallest and largest number that got us
        to a state with zero."""
        zero_states = self.states[self.states["z"] == 0]
        result_min = zero_states["min"].min()
        result_max = zero_states["max"].max()
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
            self.states[dest] = instr_func(self.states[dest], self.states[src])
        else:
            self.states[dest] = instr_func(self.states[dest], int(src))

    def execute_input(self, line):
        """Execute the `inp` instruction."""
        self.digit += 1
        print(f"=== digit {self.digit} ===")

        instr, dest = line.split()
        assert instr == "inp"
        assert dest in REGISTERS
        # first, reduce the number of states by deduplication
        with Timer() as t:
            self.deduplicate()
        print(f"deduplication: {t.elapsed:.3f} s")
        # second, make a copy of all states and set the input digit
        with Timer() as t:
            self.expand_states_into(dest)
        print(f"expansion: {t.elapsed:.3f} s")
        print("continuing with", len(self.states), "states")

    def deduplicate(self):
        groups = self.states.groupby(["x", "y", "z", "w"], sort=False)
        self.states = groups.aggregate(
            {"x": "first", "y": "first", "z": "first", "w": "first",
             "min": "min", "max": "max"})
        self.states.reset_index(drop=True, inplace=True)

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

from pathlib import Path

INPUT_FILE = Path("input.txt")
alu = NonDeterministicALU()
with Timer() as t:
    alu.run(INPUT_FILE.read_text())

print(f"Computation finished in {t.elapsed:.3f} s")

result_min, result_max = alu.find_puzzle_answer()
print("Minimum:", result_min)
print("Maximum:", result_max)
