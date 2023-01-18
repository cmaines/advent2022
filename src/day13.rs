use std::{str::Chars, iter::Peekable, cmp::Ordering};
use Entry::*;

pub fn day_13() {
  let pairs = execute();

  let mut i = 1;
  let mut valid_totals = 0;
  for (left, right) in pairs {
    let valid = is_valid(&left, &right);
    if valid == Some(true) {
      valid_totals += i;
    }
    i += 1;
  }
  println!("Result is {}", valid_totals);
}

pub fn day_13_part_2() {
  let mut entries = execute();
  let divider_2 = List(vec![
    List(vec![
      Integer(2)
    ])
  ]);
  let divider_6 = List(vec![
    List(vec![
      Integer(6)
    ])
  ]);

  entries.push((
    divider_2.clone(),
    divider_6.clone()
  ));

  let mut packets: Vec<&Entry> = entries.iter()
    .map(|(left, right)| vec![left, right])
    .flatten().collect();

    packets.sort_by(|a, b| match is_valid(a, b) {
      None => Ordering::Equal,
      Some(true) => Ordering::Less,
      _ => Ordering::Greater
    });

    let mut i = 1;
    let mut product = 1;
    for entry in packets {
      println!("{:?}", entry);
      if entry == &divider_2 || entry == &divider_6 {
        product *= i;
      }
      i += 1;
    }
    println!("Product is {}", product);

}


fn execute() -> Vec<(Entry, Entry)> {
  let mut pairs = vec![];

  let mut input: Vec<String> = vec![];
  if let Ok(lines) = super::utils::read_lines("input13.txt") {
    input.extend(lines.filter_map(|line| line.ok()));
  }

  for i in (0..input.len()).step_by(3) {
    let mut left_vec = vec![];
    let mut left_iter = &mut input[i].as_str()[0..].chars().peekable();
    get_entry(&mut left_vec, &mut left_iter);

    let mut right_vec = vec![];
    let mut right_iter = &mut input[i+1].as_str()[0..].chars().peekable();
    get_entry(&mut right_vec, &mut right_iter);

    pairs.push((left_vec.pop().unwrap(), right_vec.pop().unwrap()));
  }

  pairs
}

fn get_entry(list: &mut Vec<Entry>, iter: &mut Peekable<Chars>) { 
    while let Some(c) = iter.next() {
      match c {
        '[' => {
          let mut new_list = vec![];
          get_entry(&mut new_list, iter);
          list.push(List(new_list));
        },
        ']' => return,
        ',' => continue,
        c => {
          if c.is_digit(10) {
            let mut num = c.to_digit(10).unwrap();
            while iter.peek().unwrap().is_digit(10) {
              num *= 10;
              num += iter.next().unwrap().to_digit(10).unwrap();
            }
            list.push(Integer(num));
          }
        }
      }
  }
}

fn is_valid(left: &Entry, right: &Entry) -> Option<bool> {
  match (left, right) {
    (Integer(left_int), Integer(right_int)) => {
      if left_int == right_int {
        return None;
      }

      Some(left_int < right_int)
    },
    (Integer(left_int), List(_)) => is_valid(&List(vec![Integer(*left_int)]), right),
    (List(_), Integer(right_int)) => is_valid(left, &List(vec![Integer(*right_int)])),
    (List(left_list), List(right_list)) => {
      let (left_len, right_len) = (left_list.len(), right_list.len());
      let left_iter = left_list.iter();
      let right_iter = right_list.iter();
      let mut iter = left_iter.zip(right_iter);

      while let Some((left_val, right_val)) = iter.next() {
        if let Some(valid) = is_valid(left_val, right_val) {
          return Some(valid);
        }
      }

      if left_len == right_len {
        return None;
      } else if left_len < right_len {
        return Some(true);
      } else {
        return Some(false);
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Entry {
  List(Vec<Entry>),
  Integer(u32)
}