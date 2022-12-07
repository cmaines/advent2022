use std::collections::VecDeque;
use regex::Regex;

const DELIMETER: &str = " 1   2   3   4   5   6   7   8   9 ";

pub fn day_5() {
  process(process_move_line);
}

pub fn day_5_part_2() {
  process(process_move_line_9001);
}

fn process(move_processor: LineProcessor) {
  let mut stacks: Vec<VecDeque<char>> = vec![];
  if let Ok(line_iter) = super::utils::read_lines("input5.txt") {
    let lines: Vec<String> = line_iter.filter_map(|line| line.ok()).collect();
    let line = &lines[0];
    
    let num_stacks = (line.len() + 1) / 4;
    for _ in 0..num_stacks {
      stacks.push(VecDeque::new());
    }

    let mut processor: LineProcessor = process_stack_line;

    let mut lines_iter = lines.iter();
    while let Some(line) = lines_iter.next() {
      if line == DELIMETER {
        processor = move_processor;
        lines_iter.next();
        continue;
      }

      processor(&mut stacks, line);
    }

    print!("Result is: ");
    for stack in stacks {
      print!("{}", stack.get(0).unwrap());
    }
  }
}

fn process_stack_line(stacks: &mut Vec<VecDeque<char>>, line: &String) {
  let re = Regex::new("(...) {0,1}").unwrap();
  let captures = re.captures_iter(line.as_str());
  let mut i = 0;

  for capture_groups in captures {
    if let Some(capture) = capture_groups.get(1) {
      let capture_str = capture.as_str();
      if capture_str != "   " {
        stacks[i].push_back(capture_str.chars().nth(1).unwrap());
      }
    }
    i += 1;
  }
}
  
fn process_move_line(stacks: &mut Vec<VecDeque<char>>, line: &String) {
  let words: Vec<&str> = line.split_whitespace().collect();
  let count: u8 = words[1].parse().unwrap();
  let start: usize = words[3].parse().unwrap();
  let end: usize = words[5].parse().unwrap();

  for _ in 0..count {
    let pop = stacks[start - 1].pop_front().unwrap();
    stacks[end - 1].push_front(pop);
  }
}

fn process_move_line_9001(stacks: &mut Vec<VecDeque<char>>, line: &String) {
  let words: Vec<&str> = line.split_whitespace().collect();
  let count: usize = words[1].parse().unwrap();
  let start: usize = words[3].parse().unwrap();
  let end: usize = words[5].parse().unwrap();

  stacks[start - 1]
    .drain(0..count)
    .rev()
    .collect::<Vec<char>>()
    .iter()
    .for_each(|gnome_crate| stacks[end - 1].push_front(gnome_crate.to_owned()));
}

type LineProcessor = fn(&mut Vec<VecDeque<char>>, &String);