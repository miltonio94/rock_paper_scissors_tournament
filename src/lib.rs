use std::{error::Error, fs, process};

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let rounds = parse_file_content(&content);
    let total_score: u32 = rounds.iter().map(|r| r.score).sum();

    println!("Your total score is {total_score}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum WinState {
    Won,
    Lost,
    Drew,
}

impl WinState {
    pub fn points_from_state(self: &Self) -> u32 {
        match self {
            Self::Won => 6,
            Self::Drew => 3,
            Self::Lost => 0,
        }
    }
}

fn parse_file_content(content: &str) -> Vec<Round> {
    content
        .trim()
        .split('\n')
        .map(|line| parse_choice(line))
        .map(|choice| decide_round(choice))
        .collect()
}

#[derive(Debug, Clone)]
struct Choice(Move, Move);

impl Move {
    pub fn clone(self: &Self) -> Move {
        match self {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        }
    }
}

impl Choice {
    pub fn get_player_move(self: &Self) -> &Move {
        &self.1
    }

    pub fn get_oponent_move(self: &Self) -> &Move {
        &self.0
    }

    pub fn get_score_from_move(self: &Self) -> u32 {
        let player_move = self.get_player_move();

        match player_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn clone(self: &Self) -> Choice {
        Choice(
            self.get_oponent_move().clone(),
            self.get_player_move().clone(),
        )
    }
}

struct Round {
    choice: Choice,
    win_state: WinState,
    score: u32,
}

fn parse_choice(coded_moves: &str) -> Choice {
    let coded_moves: Vec<&str> = coded_moves.split_whitespace().collect();
    let oponet_choice = decode_move(coded_moves[0]);
    let player_choice = decode_move(coded_moves[1]);

    Choice(oponet_choice, player_choice)
}

fn decode_move(coded_move: &str) -> Move {
    match coded_move {
        "A" | "X" => Move::Rock,
        "Y" | "B" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => {
            eprint!("Error: Could not parse choice into a Move");
            process::exit(1)
        }
    }
}

fn decide_round(choice: Choice) -> Round {
    let win_state = win_state(&choice);
    let points_from_state = win_state.points_from_state();
    let score_from_move = choice.get_score_from_move();
    let score = points_from_state + score_from_move;

    Round {
        choice: choice.clone(),
        win_state,
        score,
    }
}

fn win_state(choice: &Choice) -> WinState {
    match choice {
        Choice(Move::Rock, Move::Scissors)
        | Choice(Move::Scissors, Move::Paper)
        | Choice(Move::Paper, Move::Rock) => WinState::Lost,
        Choice(Move::Paper, Move::Paper)
        | Choice(Move::Rock, Move::Rock)
        | Choice(Move::Scissors, Move::Scissors) => WinState::Drew,
        _ => WinState::Won,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn able_to_parse_choice() {
        // assert_eq!(parse_choice("AY"), Choice(Move::Rock(1), Move::Paper(2)));
    }
}
