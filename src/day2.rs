use std::{collections::HashMap, str::FromStr};

pub fn day_2() {
    let win_points = 6;
    let draw_points = 3;

    let rock = Play { points: 1, beats: Move::Scissors, loses: Move::Paper };
    let paper: Play = Play { points: 2, beats: Move::Rock, loses: Move::Scissors };
    let scissors: Play = Play { points: 3, beats: Move::Paper, loses: Move::Rock };

    let mut plays = HashMap::new();
    plays.insert(Move::Rock, &rock);
    plays.insert(Move::Paper, &paper);
    plays.insert(Move::Scissors, &scissors);

    let mut score: u32 = 0;

    if let Ok(lines) = super::utils::read_lines("input2.txt") {
        for line in lines {
            if let Ok(str) = line {
                let params: Vec<&str> = str.split_whitespace().collect();
                let opponent = plays.get(&Move::from_str(params[0]).unwrap()).unwrap();
                let win_condition: WinCondition = WinCondition::from_str(params[1]).unwrap();
                
                score += match win_condition {
                    WinCondition::Lose => plays.get(&opponent.beats).unwrap().points,
                    WinCondition::Draw => draw_points + opponent.points,
                    WinCondition::Win => win_points + plays.get(&opponent.loses).unwrap().points
                };
            }
        }
    }

    println!("Total score is {}", score);
}

struct Play {
    points: u32,
    loses: Move,
    beats: Move
}

#[derive(Debug)]
enum WinCondition {
    Lose,
    Draw,
    Win
}

impl FromStr for WinCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(())
        }
    }
}