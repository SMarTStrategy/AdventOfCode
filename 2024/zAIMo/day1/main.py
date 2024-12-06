def calculate_sum_differences(input_file):
    left_numbers = []
    right_numbers = []

    with open(input_file, 'r') as file:
        for line in file:
            col1, col2 = map(int, line.strip().split())
            left_numbers.append(col1)
            right_numbers.append(col2)

    left_numbers.sort()
    right_numbers.sort()

    total = 0
    for left, right in zip(left_numbers, right_numbers):
        distance = abs(left - right)
        total += distance

    return total

def calculate_similarity_score(input_file):
    left_numbers = []
    right_numbers = []

    with open(input_file, 'r') as file:
        for line in file:
            col1, col2 = map(int, line.strip().split())
            left_numbers.append(col1)
            right_numbers.append(col2)

    right_counts = {}
    for num in right_numbers:
        right_counts[num] = right_counts.get(num, 0) + 1

    total = 0
    for left_num in left_numbers:
        count_in_right = right_counts.get(left_num, 0)
        total += left_num * count_in_right

    return total

def part_1():
    example_result = calculate_sum_differences(input_file='day1/example.txt')
    result = calculate_sum_differences(input_file='day1/input.txt')

    print(f"Example sum of distances: {example_result}")
    print(f"Sum of distances: {result}")

def part_2():
    example_result = calculate_similarity_score(input_file='day1/example.txt')
    result = calculate_similarity_score(input_file='day1/input.txt')

    print(f"Example similarity score: {example_result}")
    print(f"Similarity score: {result}")

if __name__ == "__main__":
    part_1()
    part_2()