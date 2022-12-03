use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn calorie_counting() -> Result<(), &'static str> {
    let calories_file = File::open("resources/1_calories.txt").map_err(|_| "Cannot open file.")?;
    let calories_reader = BufReader::new(calories_file);

    let mut calories_totals: Vec<usize> = Vec::new();
    let mut current_count: usize = 0;

    for line in calories_reader.lines() {
        let read_amount = line.map_err(|_| "Cannot read file.")?;
        if read_amount.as_str() == "" {
            calories_totals.push(current_count);
            current_count = 0;
        } else if let Ok(cal) = read_amount.parse::<usize>() {
            current_count += cal
        } else {
            panic!("Unknown input")
        }
    }
    calories_totals.sort_by(|a, b| b.cmp(a));
    let answer_a = calories_totals[0];
    println!("Answer A: Result calories with max calories = {answer_a}");

    let answer_b: usize = calories_totals[0..3].iter().sum();
    println!("Answer B: Result calories 3 elves = {answer_b}");

    Ok(())
}
