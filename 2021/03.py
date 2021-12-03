NUMBERS = [[int(x) for x in line.strip()] for line in open("03.txt")]
TOTALS = [0] * len(NUMBERS[0])

for digits in NUMBERS:
    TOTALS = [x + y for x, y in zip(TOTALS, digits)]

gamma_digits = [1 if d > len(NUMBERS) / 2 else 0 for d in TOTALS]
gamma = int("".join(str(x) for x in gamma_digits), 2)
epsilon = ~gamma & 0b1111_1111_1111

print("Part 1:", gamma * epsilon)


def split_by_bit_at_pos(numbers, pos):
    a, b = arr = [[], []]
    for n in numbers:
        arr[n[pos]].append(n)

    if len(a) > len(b):
        return a, b
    else:
        return b, a


oxy, co2 = split_by_bit_at_pos(NUMBERS, 0)

for pos in range(1, len(NUMBERS[0])):
    if len(oxy) > 1:
        oxy, _ = split_by_bit_at_pos(oxy, pos)
    if len(co2) > 1:
        _, co2 = split_by_bit_at_pos(co2, pos)

oxy_result = int("".join(str(x) for x in oxy[0]), 2)
co2_result = int("".join(str(x) for x in co2[0]), 2)

print("Part 2:", oxy_result * co2_result)
