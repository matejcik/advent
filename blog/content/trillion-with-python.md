Title: 22 trillion parameters in Python
Date: 2022-01-01 11:00
Category: Python
Slug: trillion-py
Summary: Can we brute-force a 14-digit number in Python and 16 GB of RAM?

This year, I decided to be lazy and do [Advent of Code] in Python. I am very comfortable
in Python and didn't want to fight the language as well as the puzzles for once. For
that reason, it's the first time that I actually completed almost all the puzzles.

[Advent of Code]: https://adventofcode.com/

I originally used a [pre-trained neural net] to solve [Day 24]. By the time I understood
how the problem works, there was no programming left to do. But then I found out about
a [clever general solution] and decided to try to implement it myself.

[Day 24]: https://adventofcode.com/2021/day/24
[pre-trained neural net]: https://xkcd.com/2173/
[clever general solution]: https://www.mattkeeter.com/blog/2021-12-27-brute/

#### If you don't know about Advent of Code...

[Advent of Code] is a programming game that runs every year for the 25 days of Advent.
You're given a two-part puzzle every day. Usually, day 1 is the easier version and day 2
is much harder variant reusing the same input.

#### If you don't know about 2021 day 24...

[Day 24] defines a very simple computer. It has four integer registers and a bunch of
basic instructions: `add`, `mul`, `div`, `mod`, `eql`. There are no jumps, no loops, so
it's nothing more than a calculator. The instruction `inp` reads one digit of a 14-digit
input.

The puzzle is a [program] for this computer that does _some calculation_. This
calculation sometimes ends with a zero result. The goal is to find the maximum 14-digit
number which results in zero.

[program]: https://github.com/matejcik/advent/blob/main/2021/24.txt

Brute-forcing all numbers would take a lot of time -- there are 9^14 = 22 trillion
possibilities (digit 0 is disallowed). The standard solution is to read the puzzle
program, understand what it calculates, and use this knowledge to reduce the search
space.

#### If you don't know about the brute-force solution...


I learned about [Matt Keeter]'s solution from Reddit. Unlike the usual 

[Matt Keeter]: https://www.mattkeeter.com/blog/2021-12-27-brute/


```python
class ALU:
    def __init__(self) -> None:
        self.regs = {"x": 0, "y": 0, "z": 0, "w": 0}
        self.min = 0
        self.max = 0
```
