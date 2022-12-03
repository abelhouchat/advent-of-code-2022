use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Plays {
    Rock,
    Paper,
    Scissors,
}

enum Outcomes {
    Loss,
    Draw,
    Win,
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn get_outcome_part_one(moi: &Plays, opponent: &Plays) -> u32 {
    let score = match moi {
        Plays::Rock => match opponent {
            Plays::Rock => DRAW + ROCK,
            Plays::Paper => LOSS + ROCK,
            Plays::Scissors => WIN + ROCK,
        },
        Plays::Paper => match opponent {
            Plays::Rock => WIN + PAPER,
            Plays::Paper => DRAW + PAPER,
            Plays::Scissors => LOSS + PAPER,
        },
        Plays::Scissors => match opponent {
            Plays::Rock => LOSS + SCISSORS,
            Plays::Paper => WIN + SCISSORS,
            Plays::Scissors => DRAW + SCISSORS,
        },
    };

    return score;
}

fn get_outcome_part_two(opponent: &Plays, outcome: &Outcomes) -> u32 {
    let score = match opponent {
        Plays::Rock => match outcome {
            Outcomes::Loss => LOSS + SCISSORS,
            Outcomes::Draw => DRAW + ROCK,
            Outcomes::Win => WIN + PAPER,
        },
        Plays::Paper => match outcome {
            Outcomes::Loss => LOSS + ROCK,
            Outcomes::Draw => DRAW + PAPER,
            Outcomes::Win => WIN + SCISSORS,
        },
        Plays::Scissors => match outcome {
            Outcomes::Loss => LOSS + PAPER,
            Outcomes::Draw => DRAW + SCISSORS,
            Outcomes::Win => WIN + ROCK,
        },
    };

    return score;
}

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // PART ONE
    ////////////////////////////////////////////////////////////////////////////////

    let filename = "input.txt";

    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let opponent_plays = HashMap::from([
        ('A', Plays::Rock),
        ('B', Plays::Paper),
        ('C', Plays::Scissors),
    ]);
    let moi_plays = HashMap::from([
        ('X', Plays::Rock),
        ('Y', Plays::Paper),
        ('Z', Plays::Scissors),
    ]);

    let mut total_score: u32 = 0;
    for line in reader.lines() {
        let matchup = match line {
            Err(why) => panic!("Input is broked: {}", why),
            Ok(line) => line,
        };

        let opponent_play = match matchup.chars().nth(0) {
            Some(play) => match opponent_plays.get(&play) {
                Some(play) => play,
                None => panic!("Invalid play from opponent"),
            },
            None => panic!("Unexpected letter from opponent"),
        };
        let moi_play = match matchup.chars().nth(2) {
            Some(play) => match moi_plays.get(&play) {
                Some(play) => play,
                None => panic!("Invalid play from moi"),
            },
            None => panic!("Unexpected letter from moi"),
        };

        total_score += get_outcome_part_one(moi_play, opponent_play);
    }

    println!("Total score is {total_score}");

    ////////////////////////////////////////////////////////////////////////////////
    // PART TWO
    ////////////////////////////////////////////////////////////////////////////////

    let filename = "input.txt";

    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let opponent_plays = HashMap::from([
        ('A', Plays::Rock),
        ('B', Plays::Paper),
        ('C', Plays::Scissors),
    ]);

    let outcomes = HashMap::from([
        ('X', Outcomes::Loss),
        ('Y', Outcomes::Draw),
        ('Z', Outcomes::Win),
    ]);

    let mut total_score: u32 = 0;
    for line in reader.lines() {
        let matchup = match line {
            Err(why) => panic!("Input is broked: {}", why),
            Ok(line) => line,
        };

        let opponent_play = match matchup.chars().nth(0) {
            Some(play) => match opponent_plays.get(&play) {
                Some(play) => play,
                None => panic!("Invalid play from opponent"),
            },
            None => panic!("Unexpected letter from opponent"),
        };
        let outcome = match matchup.chars().nth(2) {
            Some(play) => match outcomes.get(&play) {
                Some(play) => play,
                None => panic!("Invalid outcome for outcome"),
            },
            None => panic!("Unexpected letter for outcome"),
        };

        total_score += get_outcome_part_two(opponent_play, outcome);
    }

    println!("Total score is {total_score}");
}
