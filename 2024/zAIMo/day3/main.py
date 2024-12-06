import re

def process_mul(input_file):
    with open(input_file, 'r') as file:
        content = file.read()

    result = 0

    # mi mancavano le regex

    for match in re.finditer(r'mul\((\d+),(\d+)\)', content):
        x, y = map(int, match.groups())
        result += x * y

    return result

def process_mull_filtered(input_file):
    with open(input_file, 'r') as file:
        content = file.read()

    result = 0
    enabled = True

    pattern =  r'(?:do|don\'t)\(\)|mul\(\d+,\d+\)'
    instructions = re.finditer(pattern, content)

    for instruction in instructions:
        instruction = instruction.group()

        if instruction == 'do()':
            enabled = True
        elif instruction == "don't()":
            enabled = False
        else:
            if enabled:
                x, y = map(int, re.findall(r'\d+', instruction))
                result += x * y

    return result

def part_1():
    example_result = process_mul(input_file='day3/example.txt')
    result = process_mul(input_file='day3/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

def part_2():
    example_result = process_mull_filtered(input_file='day3/example.txt')
    result = process_mull_filtered(input_file='day3/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

if __name__ == "__main__":
    part_1()
    part_2()
