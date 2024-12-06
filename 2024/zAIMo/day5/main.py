def _is_valid_order(pages, rules):
    for before, after in rules:
        if before in pages and after in pages:
            before_idx = pages.index(before)
            after_idx = pages.index(after)
            if before_idx > after_idx:
                return False
    return True


def sum_middle_page_numbers(input_file):
    with open(input_file) as f:
        content = f.read().strip().split('\n\n')

    rules = set()
    for line in content[0].split('\n'):
        before, after = map(int, line.split('|'))
        rules.add((before, after))

    updates = []
    for line in content[1].split('\n'):
        update = list(map(int, line.strip().split(',')))
        updates.append(update)

    middle_sum = 0
    for update in updates:
        if _is_valid_order(update, rules):
            middle_idx = len(update) // 2
            middle_sum += update[middle_idx]

    return middle_sum

def _sort_by_rules(pages, rules):
    pages = list(pages)
    changed = True
    while changed:
        changed = False
        for before, after in rules:
            if before in pages and after in pages:
                before_idx = pages.index(before)
                after_idx = pages.index(after)
                if before_idx > after_idx:
                    # SWAP (python che bel linguaggio)
                    pages[before_idx], pages[after_idx] = pages[after_idx], pages[before_idx]
                    changed = True
    return pages

def sum_middle_page_numbers_filtered(input_file):
    with open(input_file) as f:
        content = f.read().strip().split('\n\n')

    rules = set()
    for line in content[0].split('\n'):
        before, after = map(int, line.split('|'))
        rules.add((before, after))

    updates = []
    for line in content[1].split('\n'):
        update = list(map(int, line.strip().split(',')))
        updates.append(update)

    middle_sum = 0
    for update in updates:
        if not _is_valid_order(update, rules):
            sorted_update = _sort_by_rules(update, rules)
            middle_idx = len(sorted_update) // 2
            middle_sum += sorted_update[middle_idx]

    return middle_sum

def part_1():
    example_result = sum_middle_page_numbers(input_file='day5/example.txt')
    result = sum_middle_page_numbers(input_file='day5/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

def part_2():
    example_result = sum_middle_page_numbers_filtered(input_file='day5/example.txt')
    result = sum_middle_page_numbers_filtered(input_file='day5/input.txt')

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

if __name__ == "__main__":
    part_1()
    part_2()
