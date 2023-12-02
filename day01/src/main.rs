use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

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
            sum_extracted_digits(&line_content, &mut total_sum, true);
        } else {
            eprintln!("Error reading a line.");
        }
    }

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
            sum_extracted_digits(&line_content, &mut total_sum, false);
        } else {
            eprintln!("Error reading a line.");
        }
    }

    println!("Part 2: {}", total_sum);
}

fn sum_extracted_digits(line: &str, current_sum: &mut u32, digits_only: bool) {
    if digits_only {
        if let Some(digits) = extract_combined_digits(line) {
            *current_sum += digits;
        }
    } else {
        if let digits = extract_string_and_int_digits(line) {
            *current_sum += digits;
        }
    }
}

fn extract_combined_digits(input: &str) -> Option<u32> {
    let digit_string: String = input.chars().filter(|c| c.is_digit(10)).collect();

    if let Some(first_char) = digit_string.chars().next() {
        let first_digit = first_char.to_digit(10)?;

        if let Some(last_char) = digit_string.chars().last() {
            let last_digit = last_char.to_digit(10)?;

            // Combine the first and last digits as a string
            let combined_digits_str = format!("{}{}", first_digit, last_digit);

            // Parse the combined digits string into an integer
            return combined_digits_str.parse().ok();
        } else {
            // If there's no last digit, duplicate the first digit
            return Some(first_digit * 10 + first_digit);
        }
    }

    None
}

fn extract_string_and_int_digits(input: &str) -> u32 {
    let concatenated_result = format!("{}{}", first_digit(input).unwrap(), last_digit(input).unwrap());
    match concatenated_result.parse::<u32>() {
        Ok(parsed) => parsed,
        Err(_) => {
            // Handle parsing error, e.g., return a default value
            0
        }
    }
}

fn first_digit(input: &str) -> Option<u32> {
    let mut digit_word = String::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            digit_word.push(c);
            if let Some(digit) = word_to_digit(&digit_word) {
                return Some(digit);
            }
        } else if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap());
        }
    }

    None
}

fn last_digit(input: &str) -> Option<u32> {
    let mut digit_word = String::new();
    for c in input.chars().rev() {
        if c.is_alphabetic() {
            digit_word = format!("{}{}", c, digit_word);
            if let Some(digit) = word_to_digit(&digit_word) {
                return Some(digit);
            }
        } else if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap());
        }
    }

    None
}

fn word_to_digit(word: &str) -> Option<u32> {
    let substring_value_pairs = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Check if the input contains any of the specified substrings
    for (substring, value) in substring_value_pairs.iter() {
        if word.contains(substring) {
            return Some(*value);
        }
    }

    // If no matching substring is found, return None
    None
}
