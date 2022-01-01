from __future__ import annotations

import operator
from contextlib import contextmanager
from enum import Enum
from dataclasses import dataclass
from pathlib import Path
from time import time
from typing import Iterator, Callable

import numpy as np

XYZW = "xyzw"
"""Translation string for register indices"""
STATE_MIN = 4
"""Index of "minimum" field of the ALU state (minimal number from input that got us here)"""
STATE_MAX = 5
"""Index of "maximum" field of the ALU state (maximal number from input that got us here)"""

ALUState = np.array
Operator = Callable


class InstructionType(Enum):
    INPUT = 0
    LITERAL = 1
    REGISTER = 2


@dataclass
class Instruction:
    type: InstructionType
    operator: str
    dest_reg: int
    src_reg_or_val: int


INSTRUCTION_TABLE = {
    "add": np.ndarray.__iadd__,
    "mul": np.ndarray.__imul__,
    "div": np.ndarray.__ifloordiv__,
    "mod": np.ndarray.__imod__,
}


def compile(line: str) -> Instruction:
    op, *regs = line.split()
    dest_reg = XYZW.index(regs[0])
    if op == "inp":
        return Instruction(InstructionType.INPUT, op, dest_reg, 0)
    if regs[1] in XYZW:
        src_reg = XYZW.index(regs[1])
        return Instruction(InstructionType.REGISTER, op, dest_reg, src_reg)
    return Instruction(InstructionType.LITERAL, op, dest_reg, int(regs[1]))


@contextmanager
def dump_time(step: str, end: bool = True) -> Iterator[None]:
    print(f"{step}... ", end="", flush=True)
    start = time()
    yield
    stop = time()
    print(f"{stop - start:.3f} s", end="... " if not end else "\n", flush=True)


class NonDeterministicALU:
    MAX_STATES = 150_000_000

    def __init__(self, program: str) -> None:
        self.states = np.empty(shape=(self.MAX_STATES, 6), dtype=np.int64)
        self.states[0] = np.array([0, 0, 0, 0, 0, 0])
        self.states_len = 1
        self.program = [compile(line) for line in program.splitlines()]
        self.digit = 0

    def alu_update_minmax(self, state: int, other: int) -> None:
        self.states[state][STATE_MIN] = min(
            self.states[state][STATE_MIN], self.states[other][STATE_MIN]
        )
        self.states[state][STATE_MAX] = max(
            self.states[state][STATE_MAX], self.states[other][STATE_MAX]
        )

    def dedup_numpy(self) -> None:
        states = self.states[: self.states_len]
        with dump_time("uniq", end=False):
            # sort in-place with lexsort because of course this is not just sort()
            sort_order = np.lexsort(np.flip(states.T, axis=0))
            states.take(sort_order, axis=0, out=states)
            uniq_states, indices = np.unique(states[:, :4], axis=0, return_index=True)

            self.states_len = len(uniq_states)
            states[: self.states_len, :STATE_MIN] = uniq_states

        assert indices[0] == 0
        prev = 0
        for i, idx in enumerate(indices[1:]):
            states[i, STATE_MIN] = states[prev:idx, STATE_MIN].min()
            states[i, STATE_MAX] = states[prev:idx, STATE_MAX].max()
            prev = idx

        # final group:
        states[self.states_len - 1, STATE_MIN] = states[prev:, STATE_MIN].min()
        states[self.states_len - 1, STATE_MAX] = states[prev:, STATE_MAX].max()

    def dedup_hashmap(self) -> None:
        states = self.states[: self.states_len]
        known_states: dict[tuple, int] = {}
        cursor = 0
        with dump_time("hashmap", end=False):
            for idx, state in enumerate(states):
                tpl = tuple(state[:4])
                if tpl in known_states:
                    self.alu_update_minmax(known_states[tpl], idx)
                else:
                    states[cursor] = state
                    known_states[tpl] = cursor
                    cursor += 1
            self.states_len = cursor

    def set_input_digit(self, start: int, end: int, dest_reg: int, digit: int) -> None:
        """Perform the `inp w` instruction.

        Sets the value of the digit into the destination register.
        Also updates the min/max values of the state, by pushing the new digit.
        """
        assert start < end
        assert 1 <= digit <= 9
        self.states[start:end, dest_reg] = digit
        numbers = self.states[start:end, STATE_MIN:]
        self.states[start:end, STATE_MIN:] = numbers * 10 + digit
        # self.states[start:end, STATE_MIN] *= 10
        # self.states[start:end, STATE_MIN] += digit
        # self.states[start:end, STATE_MAX] *= 10
        # self.states[start:end, STATE_MAX] += digit

    def do_input(self, input: Instruction) -> None:
        with dump_time("dedup", end=False):
            self.dedup_numpy()
        # digits 2-9
        with dump_time("expansion", end=False):
            for digit in range(2, 10):
                start = (digit - 1) * self.states_len
                end = start + self.states_len
                self.states[start:end] = self.states[: self.states_len]
                self.set_input_digit(start, end, input.dest_reg, digit)
        # digit 1
        self.set_input_digit(0, self.states_len, input.dest_reg, 1)

        self.states_len *= 9

    def run_instr(self, instruction: Instruction) -> None:
        states = self.states[: self.states_len]
        match instruction:
            case Instruction(type=InstructionType.INPUT):
                self.digit += 1
                with dump_time(f"digit {self.digit}"):
                    self.do_input(instruction)
                print(f"{self.states_len} states after digit {self.digit}")
            case Instruction(InstructionType.LITERAL, "eql", dest_reg, val):
                states[:, dest_reg] = states[:, dest_reg] == val
            case Instruction(InstructionType.LITERAL, operator, dest_reg, val):
                op = INSTRUCTION_TABLE[operator]
                op(states[:, dest_reg], val)
            case Instruction(InstructionType.REGISTER, "eql", dest_reg, src_reg):
                states[:, dest_reg] = states[:, dest_reg] == states[:, src_reg]
            case Instruction(InstructionType.REGISTER, operator, dest_reg, src_reg):
                op = INSTRUCTION_TABLE[operator]
                op(states[:, dest_reg], states[:, src_reg])

    def run(self) -> tuple[int, int]:
        for instruction in self.program:
            self.run_instr(instruction)

        regz = XYZW.index("z")
        states = self.states[: self.states_len]
        z0min = states[states[:, regz] == 0, STATE_MIN].min()
        z0max = states[states[:, regz] == 0, STATE_MAX].max()
        # z0min = states[:, STATE_MIN].min()
        # z0max = states[:, STATE_MAX].max()
        return z0min, z0max


INPUT = Path("24.txt")
alu = NonDeterministicALU(INPUT.read_text())

start = time()
z0min, z0max = alu.run()
end = time()
print(f"Total time elapsed: {end - start:.4f} s")
print(f"Part 1: {z0max}")
print(f"Part 2: {z0min}")
