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

fn parse_letter(letter: &str) -> Rps {
    match letter {
        "A" => Rps::Rock,
        "X" => Rps::Rock,
        "B" => Rps::Paper,
        "Y" => Rps::Paper,
        "C" => Rps::Scissors,
        "Z" => Rps::Scissors,
        _ => panic!("Unrecognised letter")
    }
}

fn parse_rps(text: &str) -> Vec<(Rps, Rps)> {
    let mut pairs = vec![];
    for l in text.lines() {
        let mut pair = l.split(' ');
        let left = parse_letter(pair.next().unwrap());
        let right = parse_letter(pair.next().unwrap());
        pairs.push((left, right))
    }

    return pairs
}

fn determine_outcome(me: Rps, opp: Rps) -> Outcome {
    match (me, opp) {
        (Rps::Rock, Rps::Rock) => Outcome::Draw,
        (Rps::Rock, Rps::Paper) => Outcome::Lose,
        (Rps::Rock, Rps::Scissors) => Outcome::Win,
        (Rps::Paper, Rps::Rock) => Outcome::Win,
        (Rps::Paper, Rps::Paper) => Outcome::Draw,
        (Rps::Paper, Rps::Scissors) => Outcome::Lose,
        (Rps::Scissors, Rps::Rock) => Outcome::Lose,
        (Rps::Scissors, Rps::Paper) => Outcome::Win,
        (Rps::Scissors, Rps::Scissors) => Outcome::Draw,
    }
}

fn score(me: Rps, opp: Rps) -> u32 {
    let outcome = determine_outcome(me, opp);
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

    let total_score: u32 = parse_rps(contents).iter().map(|(left, right)| score(*right, *left)).sum();

    println!("total score = {}", total_score);
}
