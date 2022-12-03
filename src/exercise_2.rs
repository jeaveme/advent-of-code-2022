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

enum Outcome {
    Lose,
    Draw,
    Win,
}
impl Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown outcome value {value}"),
        }
    }
}

fn get_hand(value: &str) -> Hand {
    match value {
        "X" | "A" => Hand::Rock,
        "Y" | "B" => Hand::Paper,
        "Z" | "C" => Hand::Scissors,
        _ => panic!("Unknown hand input"),
    }
}

fn calculate_point_strategy_1(opponent_hand: Hand, my_hand: Hand) -> usize {
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

fn calculate_point_strategy_2(opponent_hand: Hand, my_outcome: Outcome) -> usize {
    match my_outcome {
        Outcome::Draw => 3 + opponent_hand as usize,
        Outcome::Win => {
            6 + (match opponent_hand {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            } as usize)
        }
        Outcome::Lose => {
            (match opponent_hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            } as usize)
        }
    }
}

pub fn rock_paper_scissors() -> Result<(), &'static str> {
    let strategy_file =
        File::open("resources/2_rock_paper_scissors.txt").map_err(|_| "Cannot read file.")?;
    let strategy_reader = BufReader::new(strategy_file);

    let mut score_strategy_1: usize = 0;
    let mut score_strategy_2: usize = 0;

    for line in strategy_reader.lines() {
        let strategy_line = line.map_err(|_| "Cannot read file.")?;
        let strategy_vector = strategy_line.split_whitespace().collect::<Vec<&str>>();
        let opponent_hand = get_hand(strategy_vector[0]);
        let my_value = strategy_vector[1];
        score_strategy_1 += calculate_point_strategy_1(opponent_hand, get_hand(my_value));
        score_strategy_2 += calculate_point_strategy_2(opponent_hand, Outcome::from(my_value));
    }
    println!("Answer A: Total score strategy 1 = {score_strategy_1}");
    println!("Answer A: Total score strategy 2 = {score_strategy_2}");
    Ok(())
}
