use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    // Open the file in read-only mode
    let file_path = "src/data.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut total_sum = 0;

    // Iterate over the lines, extract and append digits
    for line in reader.lines() {
        if let Ok(line_content) = line {
            total_sum = total_sum.add(get_id(&line_content));
        } else {
            eprintln!("Error reading a line.");
        }
    };

    println!("Part 1: {}", total_sum);
}

fn part2() {
    // Open the file in read-only mode
    let file_path = "src/data.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut total_sum = 0;

    // Iterate over the lines, extract and append digits
    for line in reader.lines() {
        if let Ok(line_content) = line {
            total_sum = total_sum.add(get_powers(&line_content));
        } else {
            eprintln!("Error reading a line.");
        }
    };

    println!("Part 1: {}", total_sum);
}


fn get_id(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(';').collect();

    for part in parts {
        let valid = is_colour_valid(part, "blue", 14) &
            is_colour_valid(part, "green", 13) &
            is_colour_valid(part, "red", 12);
        if !valid {
            return 0
        }
    }

    // Get the game
    if let Some(game_number) = extract_game_number(line) {
        return game_number
    }
    return 0
}

fn is_colour_valid(part: &str, colour: &str, value: u32) -> bool {
    let pattern = format!("{} {}", r"(\d+)", colour);
    let regex = Regex::new(&*pattern).unwrap();

    if let Some(captured) = regex.captures(part) {
        if let Some(amount) = captured.get(1) {
            if let Ok(number) = amount.as_str().parse::<u32>() {
                if number <= value {
                    return true
                }
                return false
            }
        }
    }
    return true
}

fn extract_game_number(input: &str) -> Option<u32> {
    // Define the pattern using a regular expression
    let pattern = r"Game (\d+)";

    // Create a Regex instance
    let regex = Regex::new(pattern).unwrap();

    // Match against the pattern
    if let Some(captured) = regex.captures(input) {
        // Extract the captured group
        if let Some(number_match) = captured.get(1) {
            if let Ok(number) = number_match.as_str().parse::<u32>() {
                return Some(number);
            }
        }
    }

    None
}

fn get_powers(line: &str) -> u32 {
    let green = extract_highest_number_for_color(line, "green").unwrap();
    let red = extract_highest_number_for_color(line, "red").unwrap();
    let blue = extract_highest_number_for_color(line, "blue").unwrap();

    return green * red * blue;
}

fn extract_highest_number_for_color(input: &str, color: &str) -> Option<u32> {
    let pattern = format!(r"(\d+) {}", color);
    let regex = Regex::new(&pattern).unwrap();

    let mut max_number: Option<u32> = None;

    for capture in regex.captures_iter(input) {
        if let Ok(number) = capture[1].parse::<u32>() {
            if let Some(current_max) = max_number {
                if number > current_max {
                    max_number = Some(number);
                }
            } else {
                // Initialize max_number if it's None
                max_number = Some(number);
            }
        }
    }

    max_number
}

