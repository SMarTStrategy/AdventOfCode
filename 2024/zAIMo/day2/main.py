def is_safe_report(input_file):
    with open(input_file, 'r') as file:
        reports = [list(map(int, line.strip().split())) for line in file]

    safe_count = 0
    for report in reports:
        is_valid = True

        is_increasing = report[0] < report[1]

        for i in range(len(report) - 1):
            diff = abs(report[i] - report[i + 1])

            if diff < 1 or diff > 3:
                is_valid = False
                break

            if is_increasing and report[i] >= report[i + 1]:
                is_valid = False
                break
            if not is_increasing and report[i] <= report[i + 1]:
                is_valid = False
                break

        if is_valid:
            safe_count += 1

    return safe_count

def is_safe_sequence(report):
    if len(report) < 2:
        return False

    is_increasing = report[0] < report[1]

    for i in range(len(report) - 1):
        diff = abs(report[i] - report[i + 1])
        if diff < 1 or diff > 3:
            return False
        if is_increasing and report[i] >= report[i + 1]:
            return False
        if not is_increasing and report[i] <= report[i + 1]:
            return False

    return True

def can_be_safe_with_removal(report):
    if is_safe_sequence(report):
        return True

    # tolgo uno per volta

    for i in range(len(report)):
        test_sequence = report[:i] + report[i+1:]
        if is_safe_sequence(test_sequence):
            return True

    return False

def process_safe_with_removal(input_file):
    with open(input_file, 'r') as file:
        reports = [list(map(int, line.strip().split())) for line in file]

    safe_count = sum(1 for report in reports if can_be_safe_with_removal(report))
    return safe_count

def part_1():
    example_result = is_safe_report(input_file='day2/example.txt')
    result = is_safe_report(input_file='day2/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

def part_2():
    example_result = process_safe_with_removal(input_file='day2/example.txt')
    result = process_safe_with_removal(input_file='day2/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

if __name__ == "__main__":
    part_1()
    part_2()
