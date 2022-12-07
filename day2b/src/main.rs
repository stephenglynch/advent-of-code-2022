use std::fs;

#[derive(Debug, Copy, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw
}

fn parse_left(letter: &str) -> Rps {
    match letter {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => panic!("Unrecognised letter")
    }
}

fn parse_right(letter: &str) -> Outcome {
    match letter {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unrecognised letter")
    }
}

fn parse_rps(text: &str) -> Vec<(Rps, Outcome)> {
    let mut pairs = vec![];
    for l in text.lines() {
        let mut pair = l.split(' ');
        let left = parse_left(pair.next().unwrap());
        let right = parse_right(pair.next().unwrap());
        pairs.push((left, right))
    }

    return pairs
}

fn determine_play(opp: Rps, result: Outcome) -> Rps {
    match (opp, result) {
        (Rps::Rock, Outcome::Lose) => Rps::Scissors,
        (Rps::Rock, Outcome::Draw) => Rps::Rock,
        (Rps::Rock, Outcome::Win) => Rps::Paper,
        (Rps::Paper, Outcome::Lose) => Rps::Rock,
        (Rps::Paper, Outcome::Draw) => Rps::Paper,
        (Rps::Paper, Outcome::Win) => Rps::Scissors,
        (Rps::Scissors, Outcome::Lose) => Rps::Paper,
        (Rps::Scissors, Outcome::Draw) => Rps::Scissors,
        (Rps::Scissors, Outcome::Win) => Rps::Rock,
    }
}

fn score(me: Rps, outcome: Outcome) -> u32 {
    let outcome_score = match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0
    };
    let choice_score = match me {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3
    };

    //println!("opp = {:?} me = {:?}", outcome_score, choice_score);

    return outcome_score + choice_score;
}

fn main() {
    let contents = &fs::read_to_string("input/input.txt").unwrap();

    let total_score: u32 = parse_rps(contents).iter().map(|(left, right)| score(determine_play(*left, *right), *right)).sum();

    println!("total score = {}", total_score);
}
