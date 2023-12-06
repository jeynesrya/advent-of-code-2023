use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

fn main() {
    part1();
}

fn part1() {
    // Open the file in read-only mode
    let file_path = "day04/src/data.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let parts: Vec<&str> = line_str.split(':').collect();

        let count = handle_scorecard(parts[1]);
        if count <= 1 {
            total_sum = total_sum.add(1);
            continue
        }
        total_sum = total_sum.add(2u32.pow(count-1));
    }

    println!("Part 1: {}", total_sum)
}

fn handle_scorecard(scorecard: &str) -> u32 {
    let (winners, chosen) = scorecard.split_once('|').unwrap();
    let winning_parts: Vec<_> = winners.split_whitespace().map(|c| c.parse::<u8>()).collect();
    let chosen_parts: Vec<_> = chosen.split_whitespace().map(|c| c.parse::<u8>()).collect();

    return chosen_parts.iter()
        .filter(|x| winning_parts.contains(*x))
        .count() as u32;
}