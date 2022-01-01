import operator
from copy import deepcopy

from contexttimer import Timer

REGISTERS = "xyzw"

class ALUState:
    def __init__(self):
        self.regs = {reg: 0 for reg in REGISTERS}
        self.min = 0
        self.max = 0

INSTRUCTIONS = {
    "add": operator.add,
    "mul": operator.mul,
    "div": operator.ifloordiv,
    "mod": operator.mod,
    "eql": operator.eq,
}

class NonDeterministicALU:
    def __init__(self):
        # at start, we only have one all-zero state
        self.states = [ALUState()]
        self.digit = 0

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

from pathlib import Path

INPUT_FILE = Path("input.txt")
alu = NonDeterministicALU()
alu.run(INPUT_FILE.read_text())

result_min, result_max = alu.find_puzzle_answer()
print("Minimum:", result_min)
print("Maximum:", result_max)
