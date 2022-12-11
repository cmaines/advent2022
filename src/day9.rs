use std::{str::FromStr, collections::HashSet, vec};

use Direction::*;

pub fn day_9() {
  execute(2);
}

pub fn day_9_part_2() {
  execute(10);
}

fn execute(length: usize) {
  let mut moves = vec![];
  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  visited.insert((0, 0)); // TODO

  if let Ok(lines) = super::utils::read_lines("input9.txt") {
    for line in lines {
      if let Ok(line) = line {
        let line: Vec<&str> = line.split_whitespace().collect();
        moves.push(Move::new(line));
      }
    }
  }

  let mut knots_pos = vec![(0, 0); length];
  for mov in moves {
    let move_dir = mov.0;
    let move_dist = mov.1;
    let vector = get_dir_vector(&move_dir);
    for _ in 0..move_dist {
      knots_pos[0].0 += vector.0;
      knots_pos[0].1 += vector.1;
      for i in 1..knots_pos.len() {
        let x_diff = knots_pos[i-1].0 - knots_pos[i].0;
        let y_diff = knots_pos[i-1].1 - knots_pos[i].1;
        let x_diff_abs = x_diff.abs();
        let y_diff_abs = y_diff.abs();
        let x_dir = if x_diff != 0 { x_diff_abs / x_diff } else { 0 };
        let y_dir = if y_diff != 0 { y_diff_abs / y_diff } else { 0 };

        if x_diff_abs + y_diff_abs >= 3 {
          knots_pos[i].0 += x_dir;
          knots_pos[i].1 += y_dir;
        } else if x_diff_abs == 2 {
          knots_pos[i].0 += x_dir;
        } else if y_diff_abs  == 2 {
          knots_pos[i].1 += y_dir;
        }
      }
      visited.insert(knots_pos.last().unwrap().clone());
    }
  }
  println!("Total visited nodes: {}", visited.len());
}

#[derive(Debug)]
struct Move(Direction, u32);

impl Move {
  fn new(line: Vec<&str>) -> Self {
    Move(Direction::from_str(line[0]).unwrap(), line[1].parse().unwrap())
  }
}

#[derive(Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down
}

fn get_dir_vector(dir: &Direction) -> (i32, i32) {
  match dir {
    Left => (-1, 0),
    Right => (1, 0),
    Up => (0, -1),
    Down => (0, 1)
  }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "L" => Ok(Left),
          "R" => Ok(Right),
          "U" => Ok(Up),
          "D" => Ok(Down),
          _ => Err(())
        }
    }
}