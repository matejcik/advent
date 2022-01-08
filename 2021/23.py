from __future__ import annotations

from enum import Enum
from dataclasses import dataclass
from typing import Iterator

from contexttimer import Timer

MAP = """
#############
#12w3x4y5z67#
###a#c#e#g###
  #b#d#f#h#
  #########
""".strip()

HALLWAY = "1234567"

GRAPH = {
    "1": "2",
    "2": "1w",
    "3": "wx",
    "4": "xy",
    "5": "yz",
    "6": "z7",
    "7": "6",
    "w": "23a",
    "x": "34c",
    "y": "45e",
    "z": "56g",
    "a": "wb",
    "b": "a",
    "c": "xd",
    "d": "c",
    "e": "yf",
    "f": "e",
    "g": "zh",
    "h": "g",
}


class Amphipod(Enum):
    Amber = 0
    Bronze = 1
    Copper = 2
    Desert = 3

    def move_cost(self) -> int:
        return 10 ** self.value


DESTINATIONS = {
    Amphipod.Amber: "ab",
    Amphipod.Bronze: "cd",
    Amphipod.Copper: "ef",
    Amphipod.Desert: "gh",
}

PATHS: dict[tuple[str, str], str] = {}


def fill_paths_dfs(distances: dict[tuple[str, str], str], start: str) -> None:
    distances[start, start] = ""

    queue = [start]
    while queue:
        current = queue.pop(0)
        for next in GRAPH[current]:
            if (start, next) in distances:
                continue
            distances[start, next] = distances[start, current] + next
            queue.append(next)


for item in GRAPH:
    fill_paths_dfs(PATHS, item)


@dataclass(frozen=True)
class State:
    pods: tuple[frozenset[str], frozenset[str], frozenset[str], frozenset[str]]

    @classmethod
    def from_seq(cls, seq: str) -> State:
        rooms_str = "".join(DESTINATIONS.values())
        # pod_map = { "A": Amber... }
        pod_map = {pod.name[0]: pod for pod in Amphipod}
        # seq =       "ACBCED..."
        # rooms_str = "abcdefgh"
        # pod_list =  [(Amber, "a"), (Copper, "b"), (Bronze, "c")...]
        pod_list = [
            (pod_map[pod_name], place) for pod_name, place in zip(seq, rooms_str)
        ]
        pods = tuple(
            # filter out places for each amphipod type
            frozenset(place for p, place in pod_list if p == pod) for pod in Amphipod
        )
        return cls(pods)

    def __str__(self) -> str:
        map = MAP
        for pod, pods in zip(Amphipod, self.pods):
            for place in pods:
                map = map.replace(place, pod.name[0])
        for place in GRAPH:
            map = map.replace(place, ".")
        return map

    def __contains__(self, place: str) -> bool:
        return any(place in pods for pods in self.pods)

    def which(self, place: str) -> Amphipod:
        for a, pods in zip(Amphipod, self.pods):
            if place in pods:
                return a
        raise ValueError(f"{place} is not occupied")

    def move(self, start: str, dest: str) -> State:
        new_state = tuple(
            (pods - {start} | {dest} if start in pods else pods) for pods in self.pods
        )
        return State(new_state)

    def count_room(self, room: str, pods: frozenset[str]) -> tuple[int, int]:
        seek = "ready"
        finished_pods = 0
        ready_spaces = 0
        for place in room:
            if seek == "ready":
                if place not in self:
                    ready_spaces += 1
                else:
                    seek = "finished"
            if seek == "finished":
                if place in pods:
                    finished_pods += 1
                else:
                    seek = None
            if seek == None:
                return 0, 0
        return finished_pods, ready_spaces

    def _yield_move(
        self, move_cost: int, start: str, dest: str
    ) -> Iterator[tuple[State, int]]:
        assert start != dest
        path = PATHS[start, dest]
        cost = len(path) * move_cost
        if not any(p in self for p in path):
            yield self.move(start, dest), cost

    def make_moves(self) -> Iterator[tuple[State, int]]:
        for a, pods in zip(Amphipod, self.pods):
            room = DESTINATIONS[a]
            move_cost = a.move_cost()
            _, ready = self.count_room(room, pods)
            if not ready:
                # move out first pod in room
                for place in room:
                    if place not in self:
                        continue
                    break
                else:
                    raise ValueError("ready is 0 for an empty room")
                # place now holds first non-empty place in room
                which = self.which(place)
                for destination in HALLWAY:
                    yield from self._yield_move(which.move_cost(), place, destination)

            else:
                dest = room[ready - 1]
                for place in pods:
                    if place in HALLWAY:
                        yield from self._yield_move(move_cost, place, dest)

    def value(self) -> int:
        result = 0
        for pod, room in DESTINATIONS.items():
            move_cost = pod.move_cost()
            pods = self.pods[pod.value]
            finished, ready = self.count_room(room, pods)
            result += finished * move_cost
            result += ready * move_cost // 2
            for place in pods:
                if place in HALLWAY:
                    # quarter points for every pod in hallway
                    result += move_cost // 4
        return result

    def straight_path_cost(self) -> int:
        total_cost = 0
        for pod, pods in zip(Amphipod, self.pods):
            move_cost = pod.move_cost()
            room = DESTINATIONS[pod]
            pods_outside = pods - set(room)
            outside_iter = iter(pods_outside)
            for place in room:
                if place not in pods:
                    total_cost += move_cost * len(PATHS[place, next(outside_iter)])
        return total_cost


FINAL_STATE = State.from_seq("AABBCCDD")


def search(start: State) -> int:
    queue: list[tuple[int, int, int, State]] = [(0, 0, 0, start)]
    visited: dict[State, tuple[int, State]] = {start: (0, start)}
    while queue:
        queue.sort(key=lambda e: e[0] + e[1])
        straight, cost, move, current = queue.pop(0)
        if current == FINAL_STATE:
            states = []
            while current != start:
                states.append((move, current))
                move, current = visited[current]
            print(start)
            for move, st in reversed(states):
                print(f"==== for {move}")
                print(st)
            return cost
        for next_state, move_cost in current.make_moves():
            new_cost = cost + move_cost
            if next_state in visited:
                continue
            visited[next_state] = move_cost, current
            queue.append(
                (next_state.straight_path_cost(), new_cost, move_cost, next_state)
            )
    raise ValueError("no solution")


INPUT_TEST = State.from_seq("BACDBCDA")

with Timer() as t:
    print(f"Test: {search(INPUT_TEST)}")
print(f"Test: {t.elapsed:.3f} s")

INPUT = State.from_seq("ABDCADBC")

with Timer() as t:
    print(f"Part 1: {search(INPUT)}")
print(f"Part 1: {t.elapsed:.3f} s")


# ==== PART 2 ==== redefine map

MAP = """
#############
#12w3x4y5z67#
###a#e#i#m###
  #b#f#j#n#
  #c#g#k#o#
  #d#h#l#p#
  #########
""".strip()

GRAPH.update({
    "b": "ac",
    "c": "bd",
    "d": "c",
    "e": "xf",
    "f": "eg",
    "g": "fh",
    "h": "g",
    "i": "yj",
    "j": "ik",
    "k": "jl",
    "l": "k",
    "m": "zn",
    "n": "mo",
    "o": "np",
    "p": "o",
    "x": "34e",
    "y": "45i",
    "z": "56m",
})

PATHS.clear()

for item in GRAPH:
    fill_paths_dfs(PATHS, item)

DESTINATIONS = {
    Amphipod.Amber: "abcd",
    Amphipod.Bronze: "efgh",
    Amphipod.Copper: "ijkl",
    Amphipod.Desert: "mnop",
}

INPUT2 = State.from_seq("ADDBDCBCABADBACC")
FINAL_STATE = State.from_seq("AAAABBBBCCCCDDDD")

with Timer() as t:
    print(f"Part 2: {search(INPUT2)}")
print(f"Part 2: {t.elapsed:.3f} s")
