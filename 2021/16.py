from __future__ import annotations
from dataclasses import dataclass
from typing import Protocol, TypeVar
from textwrap import indent
from functools import reduce
from operator import mul

T = TypeVar("T")


def bytes_to_bitstring(data: bytes) -> str:
    return "".join(f"{b:08b}" for b in data)


class BitReader:
    def __init__(self, bits: str) -> None:
        if not all(x in "01" for x in bits):
            raise ValueError("invalid bit string")
        self.bits = bits
        self.pos = 0

    def read(self, n: int) -> int:
        return int(self.read_bits(n), 2)

    def read_bits(self, n: int) -> str:
        bits = self.bits[self.pos : self.pos + n]
        self.pos += n
        return bits

    @property
    def remaining(self) -> int:
        return len(self.bits) - self.pos


class Packet(Protocol):
    version: int
    type_id: int

    @classmethod
    def parse(cls: type[T], version: int, type_id: int, reader: BitReader) -> T:
        ...

    def version_sum(self) -> int:
        ...

    def calculate(self) -> int:
        ...


@dataclass
class LiteralPacket:
    version: int
    type_id: int
    value: int

    @classmethod
    def parse(cls, version: int, type_id: int, reader: BitReader) -> LiteralPacket:
        assert type_id == 4
        value = 0
        cont_bit = 1
        while cont_bit:
            cont_bit = reader.read(1)
            value = (value << 4) | reader.read(4)

        return cls(version, type_id, value)

    def version_sum(self) -> int:
        return self.version

    def calculate(self) -> int:
        return self.value

    def __repr__(self) -> str:
        return f"LiteralPacket(version={self.version}, type_id={self.type_id}, value={self.value})"


@dataclass
class OperatorPacket:
    version: int
    type_id: int
    subpackets: list[Packet]

    @classmethod
    def parse(cls, version: int, type_id: int, reader: BitReader) -> OperatorPacket:
        subpackets = []
        length_type = reader.read(1)
        if length_type == 0:
            bit_length = reader.read(15)
            subreader = BitReader(reader.read_bits(bit_length))
            while subreader.remaining:
                subpackets.append(parse_packet(subreader))

        else:
            packets = reader.read(11)
            for _ in range(packets):
                subpackets.append(parse_packet(reader))

        return cls(version, type_id, subpackets)

    def version_sum(self) -> int:
        return self.version + sum(p.version_sum() for p in self.subpackets)

    def calculate(self) -> int:
        raise NotImplementedError

    def __repr__(self) -> str:
        header = f"{self.__class__.__name__}(version={self.version}, type_id={self.type_id})"
        subpackets = "\n".join(repr(p) for p in self.subpackets)
        return header + "\n" + indent(subpackets, "    ")


class SumPacket(OperatorPacket):
    def calculate(self) -> int:
        return sum(p.calculate() for p in self.subpackets)


class ProductPacket(OperatorPacket):
    def calculate(self) -> int:
        return reduce(mul, (p.calculate() for p in self.subpackets))


class MinimumPacket(OperatorPacket):
    def calculate(self) -> int:
        return min(p.calculate() for p in self.subpackets)


class MaximumPacket(OperatorPacket):
    def calculate(self) -> int:
        return max(p.calculate() for p in self.subpackets)


class GreaterThanPacket(OperatorPacket):
    def calculate(self) -> int:
        assert len(self.subpackets) == 2
        return self.subpackets[0].calculate() > self.subpackets[1].calculate()


class LessThanPacket(OperatorPacket):
    def calculate(self) -> int:
        assert len(self.subpackets) == 2
        return self.subpackets[0].calculate() < self.subpackets[1].calculate()


class EqualsPacket(OperatorPacket):
    def calculate(self) -> int:
        assert len(self.subpackets) == 2
        return self.subpackets[0].calculate() == self.subpackets[1].calculate()


PACKET_TYPES = {
    0: SumPacket,
    1: ProductPacket,
    2: MinimumPacket,
    3: MaximumPacket,
    4: LiteralPacket,
    5: GreaterThanPacket,
    6: LessThanPacket,
    7: EqualsPacket,
}


def parse_packet(reader: BitReader) -> Packet:
    version = reader.read(3)
    type_id = reader.read(3)
    cls = PACKET_TYPES[type_id]
    return cls.parse(version, type_id, reader)


with open("16.txt") as f:
    hex_data = f.read()
    DATA = bytes.fromhex(hex_data)

reader = BitReader(bytes_to_bitstring(DATA))
root_packet = parse_packet(reader)
print(repr(root_packet))
print(f"Part 1: {root_packet.version_sum()}")
print(f"Part 2: {root_packet.calculate()}")
