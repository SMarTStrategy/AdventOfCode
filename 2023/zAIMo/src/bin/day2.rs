use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    //let file = File::open("assets/testing.txt")?;
    let file = File::open("assets/advent_of_code_2.txt")?;
    let mut reader = BufReader::new(file);
    let total_part_1 = part_1(reader);
    reader = BufReader::new(File::open("assets/advent_of_code_2.txt")?);
    let total_part_2 = part_2(reader);
    println!("Total part 1: {}", total_part_1);
    println!("Total part 2: {}", total_part_2);
    Ok(())
}

fn part_1(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let lines = reader.lines();
    let bag_hash = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    for line in lines {
        let mut is_playable = true;
        let line = line.unwrap();
        let mut parts = line.split(":");
        let sequence = parts.next().unwrap();
        let sequence = sequence.trim();
        let sequence = sequence.split(" ").last().unwrap();
        let parts = parts.next().unwrap().split(";");
        for part in parts {
            println!("Part: {}", part);
            let colors = part.split(",");
            for color in colors {
                let mut color = color.trim().split(" ");
                let number = color.next().unwrap().parse::<u32>().unwrap();
                let color = color.next().unwrap();
                let color = color.trim();
                println!("Color: {}", color);
                println!("Number: {}", number);
                if bag_hash.contains_key(color) {
                    let value = bag_hash.get(color).unwrap();
                    println!("Value: {}", value);
                    if number > *value {
                        println!("Game can't be played, not enough {} dices", color);
                        is_playable = false;
                    }
                }
            }
        }
        if is_playable {
            total += sequence.parse::<u32>().unwrap();
        }
        println!("Sequence: {}", sequence);
        println!("--------------------------------------------");
    }
    total
}

fn part_2(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let lines = reader.lines();

    for line in lines {
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;
        let line = line.unwrap();
        let mut parts = line.split(":").skip(1);
        let parts = parts.next().unwrap().split(";");
        for part in parts {
            println!("Part: {}", part);
            let colors = part.split(",");
            for color in colors {
                let mut color = color.trim().split(" ");
                let number = color.next().unwrap().parse::<u32>().unwrap();
                let color = color.next().unwrap();
                let color = color.trim();
                match color {
                    "red" => {
                        if number > max_red {
                            max_red = number;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            max_green = number;
                        }
                    },
                    "blue" => {
                        if number > max_blue {
                            max_blue = number;
                        }
                    },
                    _ => (),
                }
                println!("Color: {}", color);
                println!("Number: {}", number);
            }
        }
        println!("Max red: {}", max_red);
        println!("Max green: {}", max_green);
        println!("Max blue: {}", max_blue);
        println!("--------------------------------------------");
        total += max_red * max_green * max_blue;
    }

    total
}