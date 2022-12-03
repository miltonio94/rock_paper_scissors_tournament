use std::{error::Error, fs, process};

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let rounds_part1 = parse_file_content_part1(&content);
    let rounds_part2 = parse_file_content_part2(&content);
    let total_score_part1: u32 = rounds_part1.iter().map(|r| r.score).sum();
    let total_score_part2: u32 = rounds_part2.iter().map(|r| r.score).sum();

    println!("Your total score for part 1 is {total_score_part1}");
    println!("Your total score for part 2 is {total_score_part2}");

    Ok(())
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
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

    pub fn clone(self: &Self) -> Self {
        match self {
            Self::Won => Self::Won,
            Self::Drew => Self::Drew,
            Self::Lost => Self::Lost,
        }
    }
}

fn parse_file_content_part1(content: &str) -> Vec<Round> {
    content
        .trim()
        .split('\n')
        .map(|line| parse_choice_part1(line))
        .map(|choice| decide_round_part1(choice))
        .collect()
}

fn parse_file_content_part2(content: &str) -> Vec<Round> {
    content
        .trim()
        .split('\n')
        .map(|line| parse_choice_part2(line))
        .map(|choice| decide_round_part1(choice))
        .collect()
}

#[derive(Clone)]
struct Choices(Move, Move);

#[derive(Clone)]
struct OponentChoiceWinState(Move, WinState);

impl OponentChoiceWinState {
    pub fn get_oponent_move(self: &Self) -> Move {
        self.0.clone()
    }

    pub fn get_win_state(self: &Self) -> WinState {
        self.1.clone()
    }

    pub fn to_choice(self: &Self) -> Choices {
        let player_choice = match self.get_win_state() {
            WinState::Won => self.get_oponent_move().move_to_win(),
            WinState::Lost => self.get_oponent_move().move_to_loose(),
            WinState::Drew => self.get_oponent_move().move_to_draw_round(),
        };

        Choices(self.get_oponent_move(), player_choice)
    }

    pub fn clone(self: &Self) -> Self {
        Self(self.get_oponent_move(), self.get_win_state())
    }
}

impl Move {
    pub fn clone(self: &Self) -> Move {
        match self {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        }
    }

    pub fn move_to_draw_round(self: &Self) -> Self {
        self.clone()
    }

    pub fn move_to_win(self: &Self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn move_to_loose(self: &Self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl Choices {
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

    pub fn clone(self: &Self) -> Choices {
        Self(
            self.get_oponent_move().clone(),
            self.get_player_move().clone(),
        )
    }
}

struct Round {
    choice: Choices,
    win_state: WinState,
    score: u32,
}

fn parse_choice_part1(coded_moves: &str) -> Choices {
    let coded_moves: Vec<&str> = coded_moves.split_whitespace().collect();
    let oponet_choice = decode_move(coded_moves[0]);
    let player_choice = decode_move(coded_moves[1]);

    Choices(oponet_choice, player_choice)
}

fn parse_choice_part2(coded_moves: &str) -> Choices {
    let coded_moves: Vec<&str> = coded_moves.split_whitespace().collect();
    let oponent_move_win_state = OponentChoiceWinState(
        decode_move(coded_moves[0]),
        decode_win_state(coded_moves[1]),
    );

   oponent_move_win_state.to_choice()
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

fn decode_win_state(coded_win_state: &str) -> WinState {
    match coded_win_state {
        "X" => WinState::Lost,
        "Y" => WinState::Drew,
        "Z" => WinState::Won,
        &_ => {
            eprint!("Error: Could not parse win state into a WinState");
            process::exit(1)
        }
    }
}

fn decide_round_part1(choice: Choices) -> Round {
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

fn win_state(choice: &Choices) -> WinState {
    match choice {
        Choices(Move::Rock, Move::Scissors)
        | Choices(Move::Scissors, Move::Paper)
        | Choices(Move::Paper, Move::Rock) => WinState::Lost,

        Choices(Move::Paper, Move::Paper)
        | Choices(Move::Rock, Move::Rock)
        | Choices(Move::Scissors, Move::Scissors) => WinState::Drew,
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
