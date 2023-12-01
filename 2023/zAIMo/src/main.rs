use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    //let file = File::open("assets/testing.txt")?;
    let file = File::open("assets/advent_of_code_1.txt")?;
    let mut reader = BufReader::new(file);
    let total_part_1 = part_1(reader);
    reader = BufReader::new(File::open("assets/advent_of_code_1.txt")?);
    let total_part_2 = part_2(reader);
    println!("Total part 1: {}", total_part_1);
    println!("Total part 2: {}", total_part_2);
    Ok(())
}

fn part_1(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let digit_re = Regex::new(r"\D+").unwrap();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let modified_line = digit_re.replace_all(&l, "");
                if let (Some(first_digit), Some(last_digit)) =
                    (modified_line.chars().next(), modified_line.chars().last())
                {
                    let first_number = first_digit.to_digit(10).unwrap_or(0);
                    let last_number = last_digit.to_digit(10).unwrap_or(0);
                    let formatted_number = format!("{}{}", first_number, last_number);
                    println!("Original line: {}", l);
                    println!("Modified line: {}", modified_line);
                    println!("First digit: {}", first_number);
                    println!("Last digit: {}", last_number);
                    println!("Formatted number: {}", formatted_number);
                    println!("--------------------------------------------");
                    total += formatted_number.parse::<u32>().unwrap_or(0);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    total
}

fn part_2(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let replaced_line = replace_numbers(l.as_str());
                if let (Some(first_digit), Some(last_digit)) =
                    (replaced_line.chars().next(), replaced_line.chars().last())
                {
                    let first_number = first_digit.to_digit(10).unwrap_or(0);
                    let last_number = last_digit.to_digit(10).unwrap_or(0);
                    let formatted_number = format!("{}{}", first_number, last_number);
                    println!("Original line: {}", l);
                    println!("Replaced line: {}", replaced_line);
                    println!("First digit: {}", first_number);
                    println!("Last digit: {}", last_number);
                    println!("Formatted number: {}", formatted_number);
                    println!("--------------------------------------------");
                    total += formatted_number.parse::<u32>().unwrap_or(0);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
    total
}

fn replace_numbers(input: &str) -> String {
    let mut result = String::new();
    let mut current_word = String::new();

    let word_to_digit = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("zero", '0'),
    ];

    for c in input.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
            if let Some(digit) = word_to_digit.iter().find_map(|&(word, d)| {
                if current_word.contains(word) {
                    current_word = current_word.chars().last().unwrap().to_string();
                    Some(d)
                } else {
                    None
                }
            }) {
                result.push(digit);
            }
        } else {
            result.push(c);
        }
    }

    result
}