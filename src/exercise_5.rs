use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn init_stacks(reader: &mut BufReader<File>) -> [VecDeque<char>; 9] {
    let mut stacks_of_crates: [VecDeque<char>; 9] = Default::default();

    for i in 0..10 {
        let mut line = String::new();
        reader.read_line(&mut line).expect("Cannot read file.");

        // Skip info and blank lines
        if i > 7 {
            continue;
        }

        let mut remaining_line = line.as_mut_str();
        for (i, stack) in stacks_of_crates.iter_mut().enumerate() {
            let crate_mark = if i < 8 {
                let (current_crate, rest) = remaining_line.split_at_mut(4);
                remaining_line = rest;
                current_crate.chars().nth(1).unwrap()
            } else {
                remaining_line.chars().nth(1).unwrap()
            };

            if crate_mark != ' ' {
                stack.push_back(crate_mark);
            }
        }
    }
    stacks_of_crates
}

fn operate_9000(stacks_of_crates: &mut [VecDeque<char>; 9], qty: usize, src: usize, dst: usize) {
    for _ in 0..qty {
        let marked_crate = stacks_of_crates[src - 1].pop_front().unwrap();
        stacks_of_crates[dst - 1].push_front(marked_crate);
    }
}

fn operate_9001(stacks_of_crates: &mut [VecDeque<char>; 9], qty: usize, src: usize, dst: usize) {
    let mut moving_crates: Vec<char> = Vec::new();
    for _ in 0..qty {
        moving_crates.push(stacks_of_crates[src - 1].pop_front().unwrap());
    }
    for _ in 0..qty {
        stacks_of_crates[dst - 1].push_front(moving_crates.pop().unwrap());
    }
}

fn build_answer(stacks_of_crates: &[VecDeque<char>; 9]) -> String {
    stacks_of_crates
        .iter()
        .map(|stack| stack.front().unwrap())
        .collect::<String>()
}

pub fn stack_supplies() -> Result<(), &'static str> {
    let file = File::open("resources/5_supply_stacks.txt").map_err(|_| "Cannot open file.")?;
    let mut reader = BufReader::new(file);
    let instruction_regex: Regex =
        Regex::new(r"move (?P<qty>\d+) from (?P<src>\d+) to (?P<dst>\d+)").unwrap();

    let mut stacks_of_crates_9000 = init_stacks(&mut reader);
    let mut stacks_of_crates_9001 = stacks_of_crates_9000.clone();

    // Execute movements in each scenario
    for line in reader.lines() {
        let movement = line.map_err(|_| "Cannot read file.")?;
        match instruction_regex.captures(&movement) {
            Some(caps) => {
                let qty = caps.name("qty").unwrap().as_str().parse::<usize>().unwrap();
                let src = caps.name("src").unwrap().as_str().parse::<usize>().unwrap();
                let dst = caps.name("dst").unwrap().as_str().parse::<usize>().unwrap();
                operate_9000(&mut stacks_of_crates_9000, qty, src, dst);
                operate_9001(&mut stacks_of_crates_9001, qty, src, dst);
            }
            None => unreachable!(),
        }
    }

    let answer_a = build_answer(&stacks_of_crates_9000);
    println!("Answer A: CrateMover 9000 = {answer_a}");
    let answer_b = build_answer(&stacks_of_crates_9001);
    println!("Answer B: CrateMover 9001 = {answer_b}");

    Ok(())
}
