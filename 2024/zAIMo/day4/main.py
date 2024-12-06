import re
from typing import List

def _get_all_directions(grid: List[str]) -> List[str]:
    height = len(grid)
    width = len(grid[0])
    all_lines = []

    # HOR
    all_lines.extend(grid)

    # VER
    for col in range(width):
        vertical = ''.join(grid[row][col] for row in range(height))
        all_lines.append(vertical)

    #TL BR - Only start from top row and left column
    # INCLUDE TOP ROW
    for start_col in range(width):
        diagonal = ''
        row, col = 0, start_col
        while row < height and col < width:
            diagonal += grid[row][col]
            row += 1
            col += 1
        if len(diagonal) >= 4:
            all_lines.append(diagonal)

    # EXCLUDE TOP ROW
    for start_row in range(1, height):
        diagonal = ''
        row, col = start_row, 0
        while row < height and col < width:
            diagonal += grid[row][col]
            row += 1
            col += 1
        if len(diagonal) >= 4:
            all_lines.append(diagonal)

    # TR BL - Only start from top row and right column
    # INCLUDE TOP ROW
    for start_col in range(width-1, -1, -1):
        diagonal = ''
        row, col = 0, start_col
        while row < height and col >= 0:
            diagonal += grid[row][col]
            row += 1
            col -= 1
        if len(diagonal) >= 4:
            all_lines.append(diagonal)

    # EXCLUDE TOP ROW
    for start_row in range(1, height):
        diagonal = ''
        row, col = start_row, width-1
        while row < height and col >= 0:
            diagonal += grid[row][col]
            row += 1
            col -= 1
        if len(diagonal) >= 4:
            all_lines.append(diagonal)

    return all_lines

def find_xmas_number(input_file):
    xmas_count = 0

    with open(input_file, 'r') as file:
        grid = [line.strip() for line in file.readlines()]

    all_lines = _get_all_directions(grid)
    for line in all_lines:
        xmas_count += len(re.findall('XMAS', line))
        xmas_count += len(re.findall('SAMX', line))

    return xmas_count


def find_x_mas_number_filtered(input_file):
    x_mas_count = 0

    with open(input_file, 'r') as file:
        grid = [line.strip() for line in file.readlines()]

    height = len(grid)
    width = len(grid[0])

    for row in range(1, height - 1):
        for col in range(1, width - 1):
            # CENTER
            if grid[row][col] != 'A':
                continue

            tl_br = grid[row-1][col-1] + grid[row][col] + grid[row+1][col+1]
            tr_bl = grid[row-1][col+1] + grid[row][col] + grid[row+1][col-1]

            if ((tl_br in ['MAS', 'SAM'] and tr_bl in ['MAS', 'SAM']) or
                (tl_br in ['MAS', 'SAM'] and tr_bl in ['MAS', 'SAM'])):
                x_mas_count += 1

    return x_mas_count

def part_1():
    example_result = find_xmas_number(input_file='day4/example.txt')
    result = find_xmas_number(input_file='day4/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

def part_2():
    example_result = find_x_mas_number_filtered(input_file='day4/example.txt')
    result = find_x_mas_number_filtered(input_file='day4/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

if __name__ == "__main__":
    part_1()
    part_2()
