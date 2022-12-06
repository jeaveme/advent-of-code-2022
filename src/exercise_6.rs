use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn detect_marker(signal: &Vec<char>, length: usize) -> Option<usize> {
    for (i, segment) in signal.windows(length).enumerate() {
        let str_segment: HashSet<char> = HashSet::from_iter(segment.iter().cloned());
        if str_segment.len() == length {
            return Some(i + length);
        }
    }
    None
}

pub fn tune_trouble() -> Result<(), &'static str> {
    let file = File::open("resources/6_tuning_trouble.txt").map_err(|_| "Cannot open file.")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).expect("Cannot read file.");

    let signal: Vec<char> = line.chars().collect();

    let answer_a = detect_marker(&signal, 4).unwrap();
    println!("Answer A: start-of-packet marker found at {answer_a}");

    let answer_b = detect_marker(&signal, 14).unwrap();
    println!("Answer B: start-of-message marker found at {answer_b}");

    Ok(())
}
