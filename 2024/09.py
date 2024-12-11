from collections import defaultdict
from dataclasses import dataclass
import heapq
import typing as t

from adventlib import timeit


def uncompressed_blocks_from(source: t.Iterator[str]) -> t.Iterator[int | None]:
    file_idx = 0
    while True:
        file_len = int(next(source))
        for _ in range(file_len):
            yield file_idx
        file_idx += 1

        space_char = next(source, None)
        if space_char is None:
            return
        space_len = int(space_char)
        for _ in range(space_len):
            yield None


def compressed_blocks(data: str) -> t.Iterator[int]:
    num_files = len(data) // 2
    max_sector = 0
    data_sectors = 0
    for file_idx in uncompressed_blocks_from(iter(data)):
        max_sector += 1
        data_sectors += file_idx is not None
    blocks_forward = uncompressed_blocks_from(iter(data))
    blocks_backward = (
        file_idx
        for file_idx in uncompressed_blocks_from(reversed(data))
        if file_idx is not None
    )
    for _ in range(data_sectors):
        file_idx = next(blocks_forward)
        if file_idx is not None:
            yield file_idx
        else:
            file_idx = next(blocks_backward)
            yield num_files - file_idx


def test_compressed_blocks():
    EXAMPLE = "2333133121414131402"
    assert (
        "".join(str(f) for f in compressed_blocks(EXAMPLE))
        == "0099811188827773336446555566"
    )


@dataclass(slots=True)
class File:
    idx: int
    pos: int
    len: int


class Space(t.NamedTuple):
    pos: int
    len: int


class Defrag:
    def __init__(self, data: str):
        self.data = data
        self.filemap = []
        self.spacemap = defaultdict(list)
        data_ints = (int(d) for d in data)
        pos = 0
        file_idx = 0
        while True:
            file_len = next(data_ints)
            self.filemap.append(File(file_idx, pos, file_len))
            file_idx += 1
            pos += file_len
            space_len = next(data_ints, None)
            if space_len is None:
                break
            heapq.heappush(self.spacemap[space_len], Space(pos, space_len))
            pos += space_len

    def defrag(self) -> None:
        for file in reversed(self.filemap):
            leftmost_space = 2**31
            best_spacemap = -1
            for size in range(file.len, 10):
                if not self.spacemap[size]:
                    continue
                space_pos = self.spacemap[size][0].pos
                if space_pos > file.pos:
                    continue
                if space_pos < leftmost_space:
                    leftmost_space = space_pos
                    best_spacemap = size

            if best_spacemap == -1:
                continue

            # pop the space
            space = heapq.heappop(self.spacemap[best_spacemap])
            # move file
            file.pos = space.pos
            # update space entry
            new_space = Space(space.pos + file.len, space.len - file.len)
            heapq.heappush(self.spacemap[new_space.len], new_space)

        self.filemap.sort(key=lambda f: f.pos)

    def sectors(self) -> t.Iterator[int | None]:
        pos = 0
        for file in self.filemap:
            while pos < file.pos:
                yield None
                pos += 1
            for _ in range(file.len):
                yield file.idx
                pos += 1


def test_part2():
    EXAMPLE = "2333133121414131402"
    defragger = Defrag(EXAMPLE)
    defragger.defrag()
    out_str = "".join(str(f if f is not None else ".") for f in defragger.sectors())
    assert out_str == "00992111777.44.333....5555.6666.....8888"


@timeit()
def part1(data: str) -> int:
    return sum(idx * f for idx, f in enumerate(compressed_blocks(data)))


@timeit()
def part2(data: str) -> int:
    defragger = Defrag(data)
    defragger.defrag()
    checksum = 0
    for i, sector in enumerate(defragger.sectors()):
        if sector is not None:
            checksum += i * sector
    return checksum


if __name__ == "__main__":
    with open("input/09.txt") as f:
        data = f.read().strip()
    print(f"Part 1: {part1(data)=}")
    print(f"Part 2: {part2(data)=}")
