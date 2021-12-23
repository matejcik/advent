from __future__ import annotations

from itertools import cycle, product
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from typing import NamedTuple

"""
Player 1 starting position: 7
Player 2 starting position: 4
"""
STARTING_POSITIONS = (6, 3)  # 0-base
BOARD_SIZE = 10


# part 1
def deterministic_dice(players: list[int], target_score: int) -> tuple[int, list[int]]:
    scores = [0] * len(players)
    rolls = 0

    def dieroll():
        nonlocal rolls
        roll = rolls % 100 + 1
        rolls += 1
        return roll

    player_turns = cycle(range(len(players)))
    while max(scores) < target_score:
        up_next = next(player_turns)
        player_roll = dieroll() + dieroll() + dieroll()
        players[up_next] = (players[up_next] + player_roll) % BOARD_SIZE
        scores[up_next] += players[up_next] + 1

    return rolls, scores


num_rolls, scores = deterministic_dice(list(STARTING_POSITIONS), 1000)
losing_score = min(scores)
print(f"Part 1: {losing_score * num_rolls}")


# ====== part 2 =======


def possible_dirac_rolls() -> dict[int, int]:
    """Calculate possible totals after rolling a Dirac dice three times, and number
    of universes in which this result occurs.
    """
    rolls = Counter()
    for dice in product((1, 2, 3), repeat=3):
        rolls[sum(dice)] += 1
    return rolls


DIRAC_ROLLS = possible_dirac_rolls()
ROLL_UNIVERSES = sum(DIRAC_ROLLS.values())
assert ROLL_UNIVERSES == 27


class Configuration(NamedTuple):
    players: tuple[int, int]
    scores: tuple[int, int]

    def make_move(self, player: int, roll: int) -> Configuration:
        position = self.players[player]
        new_position = (position + roll) % BOARD_SIZE
        player_a, player_b = self.players
        score_a, score_b = self.scores
        if player == 0:
            return Configuration(
                (new_position, player_b), (score_a + new_position + 1, score_b)
            )
        else:
            return Configuration(
                (player_a, new_position), (score_a, score_b + new_position + 1)
            )

    @classmethod
    def starting(cls, players: tuple[int, int]) -> Configuration:
        return cls(players, (0, 0))


@dataclass
class QuantumBoard:
    positions: dict[Configuration, int] = field(default_factory=Counter)
    winning_score: int = 21
    winning_universes: list[int] = field(default_factory=lambda: [0, 0])

    @classmethod
    def for_players(cls, players: tuple[int, int]) -> QuantumBoard:
        qb = cls()
        qb.positions[Configuration.starting(players)] = 1
        return qb

    def one_turn(self, player: int) -> None:
        """Simulate one quantum turn.

        Roll a Dirac die three times, and for every position, add the results to all
        positions where the roll could take us.
        """
        new_positions = Counter()
        for configuration, conf_count in self.positions.items():
            for roll, roll_count in DIRAC_ROLLS.items():
                new_conf = configuration.make_move(player, roll)
                universes = roll_count * conf_count
                if max(new_conf.scores) >= self.winning_score:
                    self.winning_universes[player] += universes
                    continue
                new_positions[new_conf] += universes

        self.positions = new_positions

    def is_finished(self) -> bool:
        return not any(opt for opt in self.positions.values())


def dirac_dice(players: tuple[int, int]) -> list[int]:
    qb = QuantumBoard.for_players(players)
    player_turns = cycle(range(len(players)))
    while not qb.is_finished():
        qb.one_turn(next(player_turns))

    return qb.winning_universes


wins = dirac_dice(STARTING_POSITIONS)
print(f"Part 2: {max(wins)}")
