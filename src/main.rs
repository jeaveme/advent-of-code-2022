mod exercise_1;
mod exercise_2;
mod exercise_3;

use std::io::stdin;

use exercise_1::calorie_counting;
use exercise_2::rock_paper_scissors;
use exercise_3::reorg_rucksack;

fn main() -> Result<(), &'static str> {
    let mut input = String::new();
    println!("Insert day number to execute:");
    stdin()
        .read_line(&mut input)
        .expect("Cannot read user input");

    match input.trim() {
        "1" => calorie_counting(),
        "2" => rock_paper_scissors(),
        "3" => reorg_rucksack(),
        _ => Err("Exercise not implemented"),
    }
}
