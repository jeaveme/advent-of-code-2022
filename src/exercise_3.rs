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

fn get_dupes_priority(
    set_1: &HashSet<char>,
    set_2: &HashSet<char>,
    optional_set_3: Option<&HashSet<char>>,
) -> usize {
    let duplicated_items = set_1.intersection(&set_2);

    if let Some(set_3) = optional_set_3 {
        duplicated_items
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&set_3)
            .map(get_priority)
            .sum::<usize>()
    } else {
        duplicated_items.map(get_priority).sum::<usize>()
    }
}

pub fn reorg_rucksack() -> Result<(), &'static str> {
    let rucksack_file =
        File::open("resources/3_rucksack_reorganization.txt").map_err(|_| "Cannot open file.")?;
    let rucksack_reader = BufReader::new(rucksack_file);

    let mut total_priority: usize = 0;
    let mut total_priority_by_group: usize = 0;

    let lines = rucksack_reader.lines().collect::<Vec<Result<String, _>>>();
    let rucksack_groups = lines.chunks(3);
    for group in rucksack_groups {
        for item in group {
            let rucksack = item.as_ref().unwrap();
            let mid = rucksack.len() / 2;
            let items_compartment_1: HashSet<char> = rucksack[0..mid].chars().collect();
            let items_compartment_2: HashSet<char> = rucksack[mid..].chars().collect();
            total_priority += get_dupes_priority(&items_compartment_1, &items_compartment_2, None);
        }

        total_priority_by_group += get_dupes_priority(
            &group[0]
                .as_ref()
                .unwrap()
                .chars()
                .collect::<HashSet<char>>(),
            &group[1]
                .as_ref()
                .unwrap()
                .chars()
                .collect::<HashSet<char>>(),
            Some(
                &group[2]
                    .as_ref()
                    .unwrap()
                    .chars()
                    .collect::<HashSet<char>>(),
            ),
        );
    }

    println!("Answer A: Sum priorities duplicated items = {total_priority}");
    println!("Answer B: Sum priorities for 3-elves groups = {total_priority_by_group}");

    Ok(())
}
