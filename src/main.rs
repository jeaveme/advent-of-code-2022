mod exercise_1;
mod exercise_2;
mod exercise_3;
mod exercise_4;
mod exercise_5;
mod exercise_6;

use std::io::stdin;

use exercise_1::calorie_counting;
use exercise_2::rock_paper_scissors;
use exercise_3::reorg_rucksack;
use exercise_4::camp_cleanup;
use exercise_5::stack_supplies;
use exercise_6::tune_trouble;

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
        "4" => camp_cleanup(),
        "5" => stack_supplies(),
        "6" => tune_trouble(),
        _ => Err("Exercise not implemented"),
    }
}
