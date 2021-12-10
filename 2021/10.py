PAIRS = dict((a, b) for a, b in ("()", "[]", "{}", "<>"))
SCORE = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}
STACK_SCORE_ORDER = " )]}>"


def score_stack(stack: list[str]) -> int:
    score = 0
    for elem in reversed(stack):
        score *= 5
        score += STACK_SCORE_ORDER.index(elem)
    return score


def score_error_or_autocomplete(line: str) -> tuple[int, int]:
    stack = []
    for char in line:
        if char in PAIRS:
            stack.append(PAIRS[char])
        elif char == stack[-1]:
            stack.pop()
        else:
            return SCORE[char], 0

    return 0, score_stack(stack)


syntax_error_score = 0
stack_scores = []

with open("10.txt") as f:
    for line in f:
        line = line.strip()
        score, stack = score_error_or_autocomplete(line)
        syntax_error_score += score
        if stack:
            stack_scores.append(stack)

stack_scores.sort()

print(f"Part 1: {syntax_error_score}")
print(f"Part 2: {stack_scores[len(stack_scores) // 2]}")
