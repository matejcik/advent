from __future__ import annotations

from enum import Enum
from typing import Iterator
from itertools import product

"""
#############
#ab1c2d3e4fg#
###h#i#j#k###
  #l#m#n#o#
  #########
"""

HALLWAY = "abcdefg"

GRAPH = {
    "a": "b",
    "b": "a1",
    "c": "12",
    "d": "23",
    "e": "34",
    "f": "4g",
    "g": "f",
    "h": "1l",
    "i": "2m",
    "j": "3n",
    "k": "4o",
    "l": "h",
    "m": "i",
    "n": "j",
    "o": "k",
    "1": "bch",
    "2": "cdi",
    "3": "dej",
    "4": "efk",
}


class Amphipod(Enum):
    Amber = 1
    Bronze = 10
    Copper = 100
    Desert = 1000


DESTINATIONS = {
    Amphipod.Amber: "hl",
    Amphipod.Bronze: "im",
    Amphipod.Copper: "jn",
    Amphipod.Desert: "ko",
}

DISTANCES: dict[tuple[str, str], int] = {}


def fill_distances_dfs(distances: dict[tuple[str, str], int], start: str) -> None:
    distances[start, start] = 0

    queue = [start]
    while queue:
        current = queue.pop(0)
        for next in GRAPH[current]:
            if (start, next) not in distances:
                distances[next, start] = distances[start, next] = (
                    distances[current, start] + 1
                )
                queue.append(next)


for item in GRAPH:
    fill_distances_dfs(DISTANCES, item)


class Map:
    def __init__(self) -> None:
        self.places: dict[str, Amphipod] = {}

    def add(self, amph: Amphipod, place: str) -> None:
        if place not in GRAPH:
            raise ValueError(f"Unknown place: {place}")
        self.places[place] = amph

    def moves(self, place: str) -> Iterator[tuple[str, int]]:
        amph = self.places.get(place)
        if amph is None:
            raise ValueError(f"Nobody in place: {place}")
        if place in HALLWAY:
            destinations = DESTINATIONS[amph]
