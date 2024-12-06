import os
import sys

def create_day_folder(day_number):
    folder_name = f"day{day_number}"

    os.makedirs(folder_name, exist_ok=True)

    main_content = '''def part_1():
    example_result = None
    result = None

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

def part_2():
    example_result = None
    result = None

    print(f"Example result: {example_result}")
    print(f"Result: {result}")

if __name__ == "__main__":
    part_1()
    part_2()
'''

    with open(os.path.join(folder_name, "main.py"), "w") as f:
        f.write(main_content)

    open(os.path.join(folder_name, "example.txt"), "w").close()
    open(os.path.join(folder_name, "input.txt"), "w").close()

    print(f"Created day {day_number}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python create_day.py <day_number>")
        sys.exit(1)

    try:
        day_num = int(sys.argv[1])
        create_day_folder(day_num)
    except ValueError:
        print("Please provide a valid number")
        sys.exit(1)
