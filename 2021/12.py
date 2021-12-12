from __future__ import annotations
from typing import Protocol, Container
from collections import defaultdict


class VisitorProtocol(Protocol):
    def try_enter(self, node: str) -> bool:
        ...

    def backtrack(self, node: str) -> None:
        ...


class SimpleVisitor:
    def __init__(self) -> None:
        self.visited: set[str] = set()

    def try_enter(self, node: str) -> bool:
        if node in self.visited:
            return False
        if node.islower():
            self.visited.add(node)
        return True

    def backtrack(self, node: str) -> None:
        self.visited.discard(node)


class Part2Visitor:
    def __init__(self, allow_only_once: Container[str]) -> None:
        self.visited: set[str] = set()
        self.visited_twice: str | None = None
        self.allow_only_once = allow_only_once

    def try_enter(self, node: str) -> bool:
        if node in self.visited:
            if (
                node.islower()
                and self.visited_twice is None
                and node not in self.allow_only_once
            ):
                self.visited_twice = node
                return True
            else:
                return False

        if node.islower():
            self.visited.add(node)
        return True

    def backtrack(self, node: str) -> None:
        if node == self.visited_twice:
            self.visited_twice = None
        else:
            self.visited.discard(node)


class Graph:
    def __init__(self) -> None:
        self.graph: dict[str, set[str]] = defaultdict(set)

    def add_edge(self, u: str, v: str) -> None:
        self.graph[u].add(v)
        self.graph[v].add(u)

    @classmethod
    def from_file(cls, filename: str) -> Graph:
        g = cls()
        with open(filename) as f:
            for line in f:
                u, v = line.strip().split("-")
                g.add_edge(u, v)
        return g

    def count_paths(self, start: str, end: str, visitor: VisitorProtocol) -> int:
        if start == end:
            return 1
        if not visitor.try_enter(start):
            return 0
        return self._count_paths(start, end, visitor)

    def _count_paths(self, start: str, end: str, visitor: VisitorProtocol) -> int:
        if start == end:
            return 1
        count = 0
        for v in self.graph[start]:
            if visitor.try_enter(v):
                count += self._count_paths(v, end, visitor)
                visitor.backtrack(v)
        return count


graph = Graph.from_file("12.txt")
count = graph.count_paths("start", "end", SimpleVisitor())
print(f"Part 1: {count}")

count2 = graph.count_paths("start", "end", Part2Visitor(("start", "end")))
print(f"Part 2: {count2}")
