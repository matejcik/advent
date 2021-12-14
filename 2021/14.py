from __future__ import annotations
from dataclasses import dataclass, field
from collections import Counter


@dataclass
class Polymer:
    orig_chain: str  # carries info about first/last element
    first: str = field(init=False)
    last: str = field(init=False)
    chain: dict[tuple[str, str], int] = field(default_factory=Counter)
    rules: dict[tuple[str, str], str] = field(default_factory=dict)

    def __post_init__(self) -> None:
        self.first = self.orig_chain[0]
        self.last = self.orig_chain[-1]
        for a, b in zip(self.orig_chain, self.orig_chain[1:]):
            self.chain[(a, b)] += 1

    def add_rule(self, pair: tuple[str, str], insert: str) -> None:
        self.rules[pair] = insert

    def step(self) -> None:
        new_chain = Counter()
        for pair, insert in self.rules.items():
            if pair not in self.chain:
                continue
            pair_count = self.chain.pop(pair)
            a, b = pair
            new_chain[(a, insert)] += pair_count
            new_chain[(insert, b)] += pair_count
        self.chain.update(new_chain)

    def letter_frequencies(self) -> Counter:
        freq: dict[str, int] = Counter()
        for (a, b), count in self.chain.items():
            freq[a] += count
            freq[b] += count
        freq[self.first] += 1
        freq[self.last] += 1
        for key in freq:
            freq[key] //= 2
        return freq


with open("14.txt") as f:
    chain_def = f.readline().strip()
    polymer = Polymer(chain_def)
    f.readline()

    for line in f:
        pair, insert = line.strip().split(" -> ")
        polymer.add_rule(tuple(pair), insert)

for _ in range(10):
    polymer.step()

counts = polymer.letter_frequencies().most_common()
print(f"Part 1: {counts[0][1] - counts[-1][1]}")

for _ in range(30):
    polymer.step()

counts = polymer.letter_frequencies().most_common()
print(f"Part 2: {counts[0][1] - counts[-1][1]}")
