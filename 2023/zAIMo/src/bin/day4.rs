use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    //let file = File::open("assets/testing.txt")?;
    let file = File::open("assets/advent_of_code_4.txt")?;
    let reader = BufReader::new(file);
    let total_part_1 = part_1(reader);
    let string_input = include_str!("../../assets/advent_of_code_4.txt");
    let total_part_2 = part_2(string_input);
    println!("Total part 1: {}", total_part_1);
    println!("Total part 2: {}", total_part_2);
    Ok(())
}

fn part_1(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split(":");
        let number = parts.next().unwrap().trim().split(" ").last().unwrap();
        parts = parts.next().unwrap().split("|");
        let mut winning_numbers_vector = parts.next().unwrap().trim().split(" ").collect::<Vec<&str>>();
        winning_numbers_vector.retain(|&x| x != "");
        let mut found_numbers_vector = parts.next().unwrap().trim().split(" ").collect::<Vec<&str>>();
        found_numbers_vector.retain(|&x| x != "");
        println!("Card Number: {}", number);
        println!("Winning Numbers: {:?}", winning_numbers_vector);
        println!("Found Numbers: {:?}", found_numbers_vector);
        let count: i32 = count_matching_elements(&winning_numbers_vector, &found_numbers_vector) - 1;
        if count >= 0 {
            println!("Count: {}", count);
            println!("Value: 2 ^ {} = {}", count, i32::pow(2, count as u32) as u32);
            total += i32::pow(2, count as u32) as u32;
        }
        println!("--------------------------------------------");
    }
    total
}

fn part_2(string_input: &str) -> u32 {
    let mut number_of_cards: HashMap<usize, u32> = HashMap::new();
    let mut total = 0;
    let lines = string_input.lines();
    let line_count = lines.clone().count();

    for i in 1..=line_count {
        number_of_cards.insert(i, 1);
    }

    for (i, line) in lines.enumerate() {
        let mut parts = line.split(":");
        let number = parts.next().unwrap().trim().split(" ").last().unwrap();
        parts = parts.next().unwrap().split("|");
        let mut winning_numbers_vector = parts.next().unwrap().trim().split(" ").collect::<Vec<&str>>();
        winning_numbers_vector.retain(|&x| x != "");
        let mut found_numbers_vector = parts.next().unwrap().trim().split(" ").collect::<Vec<&str>>();
        found_numbers_vector.retain(|&x| x != "");
        println!("Card Number: {}", number);
        println!("Winning Numbers: {:?}", winning_numbers_vector);
        println!("Found Numbers: {:?}", found_numbers_vector);
        let mut count: i32 = -1;
        for number in winning_numbers_vector {
            if found_numbers_vector.contains(&number) {
                count += 1;
            }
        }

        let current_card_count = *number_of_cards.get(&(i + 1)).unwrap();
        if count > -1 {
            let count = count as usize + 2;
            for j in (i + 2)..=(i + count) {
                number_of_cards.entry(j).and_modify(|num| *num += current_card_count);
            }
        }

        println!("--------------------------------------------");
    }

    for (_, value) in number_of_cards.iter() {
        total += value;
    }

    total
}

fn count_matching_elements(vector1: &Vec<&str>, vector2: &Vec<&str>) -> i32 {
    let mut count = 0;
    for elem1 in vector1 {
        for elem2 in vector2 {
            if elem1 == elem2 {
                println!("Match: {}", elem1);
                count += 1;
                break; // Break once a match is found for the current element in vector1
            }
        }
    }

    count
}