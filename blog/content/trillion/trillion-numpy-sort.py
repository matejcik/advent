import numpy as np

from contexttimer import Timer

REGISTERS = "xyzw"
REG_SELECTOR = list(REGISTERS)

INSTRUCTIONS = {
    "add": np.ndarray.__iadd__,
    "mul": np.ndarray.__imul__,
    "div": np.ndarray.__ifloordiv__,
    "mod": np.ndarray.__imod__,
}


class NonDeterministicALU:
    MAX_STATES = 150_000_000

    def __init__(self):
        # at start, we only have one all-zero state
        dtype = [
            ("x", "i8"),
            ("y", "i8"),
            ("z", "i8"),
            ("w", "i8"),
            ("min", "i8"),
            ("max", "i8"),
        ]
        self.states = np.empty(self.MAX_STATES, dtype=dtype)
        self.states[0] = (0, 0, 0, 0, 0, 0)
        self.num_states = 1
        self.digit = 0

    def run(self, program):
        """Run whole program."""
        for line in program.splitlines():
            self.execute(line)

    def find_puzzle_answer(self):
        """Find the smallest and largest number that got us
        to a state with zero."""
        states = self.states[:self.num_states]
        zero_states = states[states["z"] == 0]
        result_min = zero_states["min"].min()
        result_max = zero_states["max"].max()
        return result_min, result_max

    def execute(self, line):
        """Execute one line of the program."""
        if line.startswith("inp"):
            # inp is handled separately
            self.execute_input(line)
            return

        states = self.states[:self.num_states]

        # split into instruction, destination register and source
        instr, dest, src = line.split()
        if instr == "eql":
            if src in REGISTERS:
                states[dest] = states[dest] == states[src]
            else:
                states[dest] = states[dest] == int(src)
        else:
            instr_func = INSTRUCTIONS[instr]
            if src in REGISTERS:
                instr_func(states[dest], states[src])
            else:
                instr_func(states[dest], int(src))

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
        print("continuing with", self.num_states, "states")

    def deduplicate(self):
        states = self.states[:self.num_states]
        states.sort()
        unique, indices = np.unique(states[REG_SELECTOR], return_index=True)
        states[:unique.size][REG_SELECTOR] = unique
        prev = 0
        assert indices[0] == 0
        # for dest, index in enumerate(indices[1:]):
        #     self.update_minmax(dest, prev, index)
        #     prev = index
        # self.update_minmax(unique.size - 1, prev, self.num_states)
        self.num_states = unique.size

    def update_minmax(self, dest, start, end):
        mins = self.states["min"]
        mins[dest] = mins[start:end].min()
        maxs = self.states["max"]
        maxs[dest] = maxs[start:end].max()

    def expand_states_into(self, dest):
        # make 8 copies
        for i in range(8):
            start = (i + 1) * self.num_states
            end = start + self.num_states
            self.states[start:end] = self.states[:self.num_states]

        # set the input digit
        for digit in range(1, 10):
            start = (digit - 1) * self.num_states
            end = start + self.num_states
            self.set_digit(self.states[start:end], dest, digit)

        self.num_states *= 9

    @staticmethod
    def set_digit(states, dest, digit):
        states[dest] = digit
        states["min"] = states["min"] * 10 + digit
        states["max"] = states["max"] * 10 + digit

from pathlib import Path

INPUT_FILE = Path("input.txt")
alu = NonDeterministicALU()
with Timer() as t:
    alu.run(INPUT_FILE.read_text())

print(f"Computation finished in {t.elapsed:.3f} s")

result_min, result_max = alu.find_puzzle_answer()
print("Minimum:", result_min)
print("Maximum:", result_max)
