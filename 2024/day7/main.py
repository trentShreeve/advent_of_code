import itertools
import operator

def parse_line(line: str):
    left, right = line.split(':')
    target = int(left.strip())
    numbers = [int(x) for x in right.strip().split()]
    return target, numbers

def concat(a, b):
    return int(str(a) + str(b))

def can_make_target(target: int, numbers: list[int]) -> bool:

    # If there's only one number, you must match the target exactly.
    if len(numbers) == 1:
        return numbers[0] == target

    ops = [operator.add, operator.mul, concat]
    # generate all sequences of operators
    for op_seq in itertools.product(ops, repeat=len(numbers)-1):
        result = numbers[0]
        for op, next_num in zip(op_seq, numbers[1:]):
            result = op(result, next_num)
            # early exit if overshoot 
            if result > target:
                break
        else:
            if result == target:
                return True
    return False

def solve_part1(lines: list[str]) -> int:
    total = 0
    for line in lines:
        target, numbers = parse_line(line)
        if can_make_target(target, numbers):
            total += target
    return total

if __name__ == '__main__':
    with open('puzzle.txt', 'r') as f:
        lines = [line.strip() for line in f if line.strip()]
    answer = solve_part1(lines)
    print(f"Part 1 answer: {answer}")
