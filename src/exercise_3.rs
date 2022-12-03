use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_priority(object: &char) -> usize {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .position(|letter| letter == *object)
        .unwrap()
        + 1
}

pub fn reorg_rucksack() -> Result<(), &'static str> {
    let rucksack_file =
        File::open("resources/3_rucksack_reorganization.txt").map_err(|_| "Cannot open file.")?;
    let rucksack_reader = BufReader::new(rucksack_file);

    let mut total_priority: usize = 0;

    for line in rucksack_reader.lines() {
        let rucksack_items = line.map_err(|_| "Cannot read file.")?;
        let compartments_capacity = rucksack_items.len() / 2;
        let items_compartment_1: HashSet<_> =
            rucksack_items[0..compartments_capacity].chars().collect();
        let items_compartment_2: HashSet<_> =
            rucksack_items[compartments_capacity..].chars().collect();
        let duplicated_priorities = items_compartment_1
            .intersection(&items_compartment_2)
            .map(get_priority)
            .sum::<usize>();
        total_priority += duplicated_priorities;
    }

    println!("Answer A: Sum priorities duplicated items = {total_priority}");

    // let answer_b: usize = calories_totals[0..3].iter().sum();
    // println!("Answer B: Result calories 3 elves = {answer_b}");

    Ok(())
}
