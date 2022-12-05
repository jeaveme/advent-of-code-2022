use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn operate_9001(stacks_of_crates: &mut [VecDeque<char>; 9], qty: usize, src: usize, dst: usize) {
    let mut tmp_marked_crates: Vec<char> = Vec::new();
    for _ in 0..qty {
        tmp_marked_crates.push(stacks_of_crates[src - 1].pop_back().unwrap());
    }
    for _ in 0..qty {
        stacks_of_crates[dst - 1].push_back(tmp_marked_crates.pop().unwrap());
    }
}

fn operate_9000(stacks_of_crates: &mut [VecDeque<char>; 9], qty: usize, src: usize, dst: usize) {
    for _ in 0..qty {
        let marked_crate = stacks_of_crates[src - 1].pop_back().unwrap();
        stacks_of_crates[dst - 1].push_back(marked_crate);
    }
}

pub fn stack_supplies() -> Result<(), &'static str> {
    let file = File::open("resources/5_supply_stacks.txt").map_err(|_| "Cannot open file.")?;
    let mut reader = BufReader::new(file);
    let hashtag_regex: Regex =
        Regex::new(r"move (?P<qty>\d+) from (?P<src>\d+) to (?P<dst>\d+)").unwrap();
    let mut stacks_of_crates_9000: [VecDeque<char>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for i in 0..10 {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|_| "Cannot read file.")?;

        if i > 7 {
            continue;
        }

        let mut remaining_line = line.as_mut_str();
        for (i, stack) in stacks_of_crates_9000.iter_mut().enumerate() {
            let crate_mark = if i < 8 {
                let (current_crate, rest) = remaining_line.split_at_mut(4);
                remaining_line = rest;
                current_crate.chars().nth(1).unwrap()
            } else {
                remaining_line.chars().nth(1).unwrap()
            };

            if crate_mark != ' ' {
                stack.push_front(crate_mark);
            }
        }
    }

    let mut stacks_of_crates_9001 = stacks_of_crates_9000.clone();

    for line in reader.lines() {
        let op = line.map_err(|_| "Cannot read file.")?;
        match hashtag_regex.captures(&op) {
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

    let answer_a = stacks_of_crates_9000
        .map(|mut stack| stack.pop_back().unwrap().to_string())
        .join("");
    println!("Answer A: {answer_a}");
    let answer_b = stacks_of_crates_9001
        .map(|mut stack| stack.pop_back().unwrap().to_string())
        .join("");
    println!("Answer B: {answer_b}");
    Ok(())
}
