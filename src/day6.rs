use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const NUM_DISTINCT: usize = 14;

pub fn day_6() {
  let mut file = File::open("input6.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut index = 0;
  let mut unique_chars: HashSet<char> = HashSet::new();
  while unique_chars.len() != NUM_DISTINCT {
    unique_chars.clear();
    unique_chars.extend(contents[index..index + NUM_DISTINCT].chars());
    index += 1;
  }
  println!("Answer is {}", index + NUM_DISTINCT - 1);
}