use std::{str::FromStr, collections::VecDeque};

use Operation::*;

pub fn day_10() {
  let mut instructions = VecDeque::new();
  if let Ok(lines) = super::utils::read_lines("input10.txt") {
    for line in lines {
      if let Ok(line) = line {
        let line: Vec<&str> = line.split_whitespace().collect();
        let op = Operation::from_str(line[0].clone()).unwrap();
        let argument: i32 = if line.len() == 2 { line[1].parse().unwrap() } else { 0 };
        instructions.push_back(Instruction { op, argument });
      }
    }
  }

  let mut tick = 1;
  let mut register = 1;

  let mut instruction = instructions.pop_front().unwrap();
  let mut ticks_remaining = get_ticks_for_instruction(&instruction.op);
  let mut sum_of_strengths = 0;
  
  loop {
    ticks_remaining -= 1;
    if (tick - 20) % 40 == 0 && tick <= 220 {
      sum_of_strengths += tick * register;
    }

    let pixel = (tick - 1) % 40;
    if pixel == 0 {
      println!();
    }

    if pixel >= register - 1 && pixel <= register + 1 {
      print!("#");
    }
    else { 
      print!(" ");
    };

    if ticks_remaining == 0 {
      match instruction.op {
        ADDX => register += instruction.argument,
        _ => { } // noop
      }
      
      if instructions.is_empty() {
        break;
      }
      instruction = instructions.pop_front().unwrap();
      ticks_remaining = get_ticks_for_instruction(&instruction.op);
    }
    tick += 1;
  }

  println!("\nSum of strengths is: {}", sum_of_strengths);
}

fn get_ticks_for_instruction (op: &Operation) -> u32 {
  match op {
    NOOP => 1,
    ADDX => 2
  }
}

#[derive(Debug)]
enum Operation {
  NOOP,
  ADDX
}

impl FromStr for Operation {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "noop" => Ok(NOOP),
      "addx" => Ok(ADDX),
      _ => Err(())
    }
  }
}

struct Instruction {
  op: Operation,
  argument: i32
}