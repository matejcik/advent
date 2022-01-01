import operator
from array import array

from contexttimer import Timer

REGISTERS = "xyzw"

INSTRUCTIONS = {
    "add": operator.add,
    "mul": operator.mul,
    "div": operator.ifloordiv,
    "mod": operator.mod,
    "eql": operator.eq,
}

STATE_MIN = 4
STATE_MAX = 5

class NonDeterministicALU:
    def __init__(self):
        # at start, we only have one all-zero state
        self.states = [array('q', [0] * 6)]
        self.digit = 0

    def run(self, program):
        """Run whole program."""
        for line in program.splitlines():
            self.execute(line)

    def find_puzzle_answer(self):
        """Find the smallest and largest number that got us
        to a state with zero."""
        reg_z = REGISTERS.index("z")
        zero_states = (state for state in self.states
                       if state[reg_z] == 0)
        result_min = 10 ** 15  # all 14-digit inputs are smaller
        result_max = 0         # all 14-digit inputs are larger
        for state in zero_states:
            # find minimum and maximum in one pass
            result_min = min(result_min, state[STATE_MIN])
            result_max = max(result_max, state[STATE_MAX])
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
        dest_reg = REGISTERS.index(dest)
        if src in REGISTERS:
            # src can either be one of x, y, z, w, or an integer
            # literal value. We expect a lot of states, so we don't
            # want to do "if src in REGISTERS" every time. So we
            # do it ahead of time and call the appropriate function.
            src_reg = REGISTERS.index(src)
            self.execute_register(instr_func, dest_reg, src_reg)
        else:
            self.execute_literal(instr_func, dest_reg, int(src))

    def execute_register(self, instr_func, dest, src):
        """Execute an instruction that takes a register as source."""
        for state in self.states:
            # call the `instr_func` on two registers
            # assign the result to dest
            state[dest] = instr_func(state[dest], state[src])

    def execute_literal(self, instr_func, dest, val):
        """Execute an instruction that takes a literal value."""
        for state in self.states:
            # call the `instr_func` on a register and value
            state[dest] = instr_func(state[dest], val)

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

    @staticmethod
    def update_minmax(state1, state2):
        state1[STATE_MIN] = min(state1[STATE_MIN], state2[STATE_MIN])
        state1[STATE_MAX] = max(state1[STATE_MAX], state2[STATE_MAX])

    def expand_states_into(self, dest):
        dest_reg = REGISTERS.index(dest)
        for i in range(len(self.states)):
            for digit in range(2, 10):
                # digits 2 to 9
                new_state = self.states[i][:]
                self.set_digit(new_state, dest_reg, digit)
                self.states.append(new_state)
            # digit 1
            self.set_digit(self.states[i], dest_reg, 1)

    @staticmethod
    def set_digit(state, dest, digit):
        state[dest] = digit
        state[STATE_MIN] = state[STATE_MIN] * 10 + digit
        state[STATE_MAX] = state[STATE_MAX] * 10 + digit

from pathlib import Path

INPUT_FILE = Path("input.txt")
alu = NonDeterministicALU()
with Timer() as t:
    alu.run(INPUT_FILE.read_text())

print(f"Computation finished in {t.elapsed:.3f} s")

result_min, result_max = alu.find_puzzle_answer()
print("Minimum:", result_min)
print("Maximum:", result_max)
