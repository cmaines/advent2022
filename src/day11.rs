use std::{str::FromStr, collections::VecDeque};
use regex::Regex;
use Operator::*;

pub fn day_11() {
  execute(20, false);
}

pub fn day_11_part_2() {
  execute(10000, true);
}

fn execute(rounds: u32, use_common_factor: bool) {
  let mut monkeys: Vec<Monkey> = vec![];
  let mut input: Vec<String> = vec![];
  let mut reduction_factor = 1;

  if let Ok(lines) = super::utils::read_lines("input11.txt") {
    input.extend(lines.filter_map(|line| line.ok()));
  }

  for i in (0..input.len()).step_by(7) {
    let items = get_starting_items(input[i+1].as_str());
    let operation = get_operation(input[i+2].as_str());
    let test_divisor: u64 = input[i+3].split_whitespace().last().unwrap().parse().unwrap();
    let true_throw: usize = input[i+4].split_whitespace().last().unwrap().parse().unwrap();
    let false_throw: usize = input[i+5].split_whitespace().last().unwrap().parse().unwrap();
    monkeys.push(Monkey { items, operation, test_divisor, true_throw, false_throw, inspections: 0 });
    reduction_factor *= test_divisor;
  }

  for _ in 0..rounds {
    let mut tosses = vec![];

    for j in 0..monkeys.len() {
      let mut monkey = &mut monkeys[j];
      let operator = &monkey.operation.operator;
      let operand = monkey.operation.operand;

      let items = &mut monkey.items;
      while !items.is_empty() {
        let mut item = items.pop_front().unwrap();
        let operand = operand.unwrap_or(item);
        item = match operator {
          ADD => item + operand,
          MULTIPLY => item * operand
        };
        item = if use_common_factor { item % reduction_factor } else { item / 3 };
        
        let monkey_toss = if item % monkey.test_divisor == 0 { monkey.true_throw } else { monkey.false_throw };
        tosses.push((monkey_toss, item));
        
        monkey.inspections += 1;
      }
      for toss in &tosses {
        monkeys[toss.0].items.push_back(toss.1);
      }
      tosses.clear();
    }
  }
  monkeys.iter().for_each(|monkey| println!("Monkey inspected items {} times", monkey.inspections));
}

fn get_operation(input: &str) -> Operation {
  let input: Vec<&str> = input.split_whitespace().collect();
  let operator = Operator::from_str(input[4]).unwrap();
  let operand = input[5].parse::<u64>().ok();
  Operation { operator, operand }
}

fn get_starting_items(input: &str) -> VecDeque<u64> {
  let re_items = Regex::new("(\\d+)").unwrap();
  let captures = re_items.captures_iter(input);

  captures.map(
    |group| group.get(0).unwrap()
      .as_str()
      .parse::<u64>()
      .unwrap()
    )
    .collect()
}

struct Monkey {
  items: VecDeque<u64>,
  operation: Operation,
  test_divisor: u64,
  true_throw: usize,
  false_throw: usize,
  inspections: u32
}

#[derive(Debug)]
struct Operation {
  operator: Operator,
  operand: Option<u64>
}

#[derive(Debug)]
enum Operator {
  ADD,
  MULTIPLY
}

impl FromStr for Operator {
  type Err = ();

fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "*" => Ok(MULTIPLY),
          "+" => Ok(ADD),
          _ => Err(())
        }
    }

  
}