use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    //let file = File::open("assets/testing.txt")?;
    let file = File::open("assets/advent_of_code_3.txt")?;
    let mut reader = BufReader::new(file);
    let total_part_1 = part_1(reader);
    reader = BufReader::new(File::open("assets/advent_of_code_3.txt")?);
    let total_part_2 = part_2(reader);
    println!("Total part 1: {}", total_part_1);
    println!("Total part 2: {}", total_part_2);
    Ok(())
}

fn part_1(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    //create 2 dimension array from file
    let mut array: Vec<Vec<char>> = Vec::new();
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        let mut line_array: Vec<char> = Vec::new();
        for c in line.chars() {
            line_array.push(c);
        }
        array.push(line_array);
    }
    let mut i = 0;
    let mut j = 0;

    while i < array.len() {
        while j < array[i].len() {
            if array[i][j].is_numeric() {
                let mut number = array[i][j].to_digit(10).unwrap();
                let mut k = j + 1;

                while k < array[i].len() && array[i][k].is_numeric() {
                    number = format!("{}{}", number, array[i][k].to_digit(10).unwrap()).parse::<u32>().unwrap();
                    k += 1;
                }

                if is_number_adjacent_to_symbol(&array, i, j, k - 1) {
                    total += number;
                }

                j = k - 1;
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }

    fn is_number_adjacent_to_symbol(array: &Vec<Vec<char>>, row: usize, start_col: usize, end_col: usize) -> bool {
        let mut is_adjacent = false;
        let mut column = start_col;

        while column < end_col + 1 {
            if has_adjacent_symbol(array, row, column) {
                is_adjacent = true;
            }
            column += 1;
        }

        return is_adjacent;
    }


    fn has_adjacent_symbol(array: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let offsets = vec![
            [-1, -1], [-1, 0], [-1, 1],
            [0, -1], [0, 1],
            [1, -1], [1, 0], [1, 1]
            ];

        for offset in offsets {
            let row = i as i32 + offset[0];
            let col = j as i32 + offset[1];
            if row >= 0 && row < array.len() as i32 && col >= 0 && col < array[row as usize].len() as i32 {
                let adjacent_char = array[row as usize][col as usize];
                if !adjacent_char.is_numeric() && adjacent_char != '.' && adjacent_char != ' ' {
                    return true;
                }
            }
        }
        return false;
    }

    total
}

fn part_2(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let mut array: Vec<Vec<char>> = Vec::new();
    let lines = reader.lines();
    for line in lines {
        let re = regex::Regex::new(r"[^0-9*]").unwrap();
        let line = line.unwrap();
        let _line = re.replace_all(&line, ".");
        let mut line_array: Vec<char> = Vec::new();
        for c in _line.chars() {
            line_array.push(c);
        }
        array.push(line_array);
    }

    let mut i = 0;
    let mut j = 0;
    let mut gear_numbers: HashMap<String, Vec<u32>> = HashMap::new();

    while i < array.len() {
        while j < array[i].len() {
            if array[i][j].is_numeric() {
                let mut number = array[i][j].to_digit(10).unwrap();
                let mut k = j + 1;

                while k < array[i].len() && array[i][k].is_numeric() {
                    number = format!("{}{}", number, array[i][k].to_digit(10).unwrap()).parse::<u32>().unwrap();
                    k += 1;
                }

                let gear_pos = get_gear_position(&array, i, j, k - 1);
                if gear_pos != "" {
                    gear_numbers.entry(gear_pos).or_insert(Vec::new()).push(number);
                }

                j = k - 1;
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }

    for gear_number in gear_numbers {
        let mut numbers: Vec<u32> = Vec::new();
        for number in gear_number.1 {
            numbers.push(number);
        }
        if numbers.len() != 2 {
            continue;
        }
        if numbers[0] != 0 && numbers[1] != 0 {
            total += numbers[0] * numbers[1];
        }
    }

    fn get_gear_position(array: &Vec<Vec<char>>, row: usize, start_col: usize, end_col: usize) -> String {
        let mut column = start_col;

        while column < end_col + 1 {
            let gear = find_gear_pos(array, row, column);
            if gear != "" {
                return gear;
            }
            column += 1;
        }

        return String::from("");
    }


    fn find_gear_pos(array: &Vec<Vec<char>>, i: usize, j: usize) -> String {
        let offsets = vec![
            [-1, -1], [-1, 0], [-1, 1],
            [0, -1], [0, 1],
            [1, -1], [1, 0], [1, 1]
            ];

        for offset in offsets {
            let row = i as i32 + offset[0];
            let col = j as i32 + offset[1];
            if row >= 0 && row < array.len() as i32 && col >= 0 && col < array[row as usize].len() as i32 {
                let adjacent_char = array[row as usize][col as usize];
                if !adjacent_char.is_numeric() && adjacent_char != '.' && adjacent_char != ' ' {
                    return format!("{}-{}", row, col);
                }
            }
        }
        return String::from("");
    }

    total
}