from __future__ import annotations

import itertools
import operator
from array import array
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
from time import time
from typing import Iterator, Callable

XYZW = "xyzw"
"""Translation string for register indices"""
STATE_MIN = 4
"""Index of "minimum" field of the ALU state (minimal number from input that got us here)"""
STATE_MAX = 5
"""Index of "maximum" field of the ALU state (maximal number from input that got us here)"""

ALUState = array
Operator = Callable[[int, int], int]


@dataclass
class Instruction:
    dest_reg: int


@dataclass
class InputInstruction(Instruction):
    def input_digit(self, state: ALUState, digit: int) -> None:
        state[self.dest_reg] = digit
        state[STATE_MIN] = state[STATE_MIN] * 10 + digit
        state[STATE_MAX] = state[STATE_MAX] * 10 + digit


@dataclass
class LiteralInstruction(Instruction):
    value: int
    operator: Operator

    def eval(self, state: ALUState) -> None:
        state[self.dest_reg] = self.operator(state[self.dest_reg], self.value)


@dataclass
class RegisterInstruction(Instruction):
    dest_reg: int
    src_reg: int
    operator: Operator

    def eval(self, state: ALUState) -> None:
        state[self.dest_reg] = self.operator(state[self.dest_reg], state[self.src_reg])


INSTRUCTION_TABLE = {
    "add": operator.add,
    "mul": operator.mul,
    "div": operator.ifloordiv,
    "mod": operator.mod,
    "eql": lambda a, b: int(a == b),
}


def compile(line: str) -> Instruction:
    op, *regs = line.split()
    dest_reg = XYZW.index(regs[0])
    if op == "inp":
        return InputInstruction(dest_reg)
    if op not in INSTRUCTION_TABLE:
        raise ValueError(f"Unknown instruction: {op}")
    if regs[1] in XYZW:
        src_reg = XYZW.index(regs[1])
        return RegisterInstruction(dest_reg, src_reg, INSTRUCTION_TABLE[op])
    return LiteralInstruction(dest_reg, int(regs[1]), INSTRUCTION_TABLE[op])


@contextmanager
def dump_time(step: str) -> Iterator[None]:
    print(f"Starting {step}...", end="", flush=True)
    start = time()
    yield
    end = time()
    print(f"{end - start:.3f} s")


class NonDeterministicALU:

    INPUT = object()

    def __init__(self, program: str) -> None:
        self.states: list[ALUState] = [array("q", [0, 0, 0, 0, 0, 0])]
        self.program = [compile(line) for line in program.splitlines()]
        self.input_ctr = 0

    @staticmethod
    def alu_update_minmax(state: ALUState, other: ALUState) -> None:
        state[STATE_MIN] = min(state[STATE_MIN], other[STATE_MIN])
        state[STATE_MAX] = max(state[STATE_MAX], other[STATE_MAX])

    def _run_chunk_on_state(self, chunk: list[Instruction], state: ALUState) -> None:
        for instruction in chunk:
            instruction.eval(state)

    def _run_chunk(self, chunk: list[Instruction]) -> None:
        last_instr = chunk[-1]
        should_expand = isinstance(last_instr, InputInstruction)
        if should_expand:
            chunk = chunk[:-1]

        known_states: dict[tuple, ALUState] = {}

        cursor = 0
        for current in range(len(self.states)):
            state = self.states[current]
            self._run_chunk_on_state(chunk, state)
            tpl = tuple(state[:4])
            if tpl in known_states:
                # we converged on one of the known states
                self.alu_update_minmax(known_states[tpl], state)
            else:
                # add new state
                known_states[tpl] = state
                assert cursor <= current
                self.states[cursor] = state
                cursor += 1

        # delete everything between where (a) append cursor is pointing (everything
        # before it was added via append(), and (b) the original length of self.states
        # (represented by `current`) - everything after it was _also_ added via append()
        del self.states[cursor:]

        if not should_expand:
            return

        print(" expanding...", end="", flush=True)
        # perform state expansion
        for i in range(len(self.states)):
            state = self.states[i]
            for digit in range(2, 10):
                new_state = state[:]
                last_instr.input_digit(new_state, digit)
                self.states.append(new_state)
            last_instr.input_digit(state, 1)


    def run(self) -> tuple[int, int]:
        chunks: list[list[Instruction]] = []
        chunk: list[Instruction] = []
        for instr in self.program:
            chunk.append(instr)
            if isinstance(instr, InputInstruction):
                chunks.append(chunk)
                chunk = []
        chunks.append(chunk)

        assert len(chunks) == 15

        for i, chunk in enumerate(chunks):
            with dump_time(f"chunk {i}"):
                self._run_chunk(chunk)
            print(f"{len(self.states)} states after chunk {i}")

        z0min = 10 ** 15
        z0max = 0
        regz = XYZW.index("z")
        for state in self.states:
            if state[regz] != 0:
                continue
            z0min = min(z0min, state[STATE_MIN])
            z0max = max(z0max, state[STATE_MAX])
        return z0min, z0max


INPUT = Path("24.txt")
alu = NonDeterministicALU(INPUT.read_text())

start = time()
z0min, z0max = alu.run()
end = time()
print(f"Total time elapsed: {end - start:.4f} s")
print(f"Part 1: {z0max}")
print(f"Part 2: {z0min}")
