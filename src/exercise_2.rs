use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn get_hand(value: &str) -> Hand {
    match value {
        "X" | "A" => Hand::Rock,
        "Y" | "B" => Hand::Paper,
        "Z" | "C" => Hand::Scissors,
        _ => panic!("Unknown hand input"),
    }
}

fn calculate_points(opponent_hand: Hand, my_hand: Hand) -> usize {
    let points: usize = if opponent_hand == my_hand {
        3
    } else {
        match (opponent_hand, my_hand) {
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => 6,
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => 0,
            (_, _) => {
                unreachable!("All cases covered")
            }
        }
    };
    return (my_hand as usize) + points;
}

pub fn rock_paper_scissors() -> Result<(), &'static str> {
    let strategy_file =
        File::open("resources/2_rock_paper_scissors.txt").map_err(|_| "Cannot read file.")?;
    let strategy_reader = BufReader::new(strategy_file);

    let mut total_score: usize = 0;

    for line in strategy_reader.lines() {
        let strategy_line = line.map_err(|_| "Cannot read file.")?;
        let strategy_vector = strategy_line.split_whitespace().collect::<Vec<&str>>();
        total_score += calculate_points(get_hand(strategy_vector[0]), get_hand(strategy_vector[1]));
    }
    println!("Answer A: Total score = {total_score}");
    Ok(())
}
