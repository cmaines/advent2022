use Direction::*;
use std::cmp::max;

pub fn day_8() {
  let mut forest = vec![];
  populate_forest(&mut forest);
  let row_count = forest.len();
  let column_count = forest[0].len();
  let mut tallest: Vec<Vec<Tallest>> = vec![vec![Tallest::default(); column_count]; row_count];
  
  let mut num_visible = 0;
  for row in 0..row_count {
    for col in 0..column_count {
      if is_visible(row, col, &forest, &mut tallest) {
        num_visible += 1;
      }
    }
  }
  println!("Number visible: {}", num_visible);
}

pub fn day_8_part_2() {
  let mut forest = vec![];
  populate_forest(&mut forest);
  let row_count = forest.len();
  let column_count = forest[0].len();
  
  let mut max_score = 0;
  for row in 0..row_count {
    for col in 0..column_count {
      max_score = max(max_score, get_scenic_score(row, col, &forest));
    }
  }
  println!("Highest scenic score: {}", max_score);
}

fn get_scenic_score(row: usize, col: usize, forest: &Vec<Vec<i32>>) -> u32 {
  let tree_height = forest[row][col];
  let dist_left = get_scenic_dist(&Left, row, col, forest, tree_height);
  let dist_top = get_scenic_dist(&Top, row, col, forest, tree_height);
  let dist_right = get_scenic_dist(&Right, row, col, forest, tree_height);
  let dist_bottom = get_scenic_dist(&Bottom, row, col, forest, tree_height);

  dist_left * dist_top * dist_right * dist_bottom
}

fn get_scenic_dist(direction: &Direction, row: usize, col: usize, forest: &Vec<Vec<i32>>, target: i32) -> u32 {
  let increments = get_increments(direction);
  let next_row = (row as i32 + increments.1 as i32) as usize;
  let next_col = (col as i32 + increments.0 as i32) as usize;
  
  let next_height = get_tree_height(next_row, next_col, forest, -1);
  if next_height == -1 {
    return 0;
  } else if next_height >= target {
    return 1;
  }
  1 + get_scenic_dist(direction, next_row, next_col, forest, target)
}

fn populate_forest(forest: &mut Vec<Vec<i32>>) {
  if let Ok(lines) = super::utils::read_lines("input8.txt") {
    for line in lines {
      if let Ok(line) = line {
        let line: Vec<char> = line.chars().collect();
        let mut row = vec![];
        for char in line {
          row.push(char.to_digit(10).unwrap() as i32);
        }
        forest.push(row);
      }
    }
  }
}

fn is_visible(row: usize, col: usize, forest: &Vec<Vec<i32>>, tallest: &mut Vec<Vec<Tallest>>) -> bool {
  let tallest_left = get_tallest(&Left, row, col, forest, tallest);
  let tallest_top = get_tallest(&Top, row, col, forest, tallest);
  let tallest_right = get_tallest(&Right, row, col, forest, tallest);
  let tallest_bottom = get_tallest(&Bottom, row, col, forest, tallest);
  
  let tree_height = forest[row][col];
  let surrounding_heights = [tallest_left, tallest_top, tallest_right, tallest_bottom];
  
  let res = surrounding_heights.iter().any(|height| *height < tree_height);
  res
}

fn get_tallest(direction: &Direction, row: usize, col: usize, forest: &Vec<Vec<i32>>, tallest: &mut Vec<Vec<Tallest>>) -> i32 {
  if row >= forest.len() || col >= forest[0].len() {
    return -1;
  }

  match get_dir_value(&direction, &tallest[row][col]) {
    Some(value) => value,
    None => {
      let increments = get_increments(&direction);
      let next_row = (row as i32 + increments.1 as i32) as usize;
      let next_col = (col as i32 + increments.0 as i32) as usize;
      let near_tallest = get_tallest(&direction, next_row, next_col, forest, tallest);
      let near_tree_height = get_tree_height(next_row, next_col, forest, -1);

      let value = max(near_tallest, near_tree_height);  
      set_dir_value(&direction, value, &mut tallest[row][col])
    }
  }
}

fn get_tree_height(row: usize, col: usize, forest: &Vec<Vec<i32>>, default: i32) -> i32 {
  if row >= forest.len() || col >= forest[0].len() {
    return default;
  }
  forest[row][col] as i32
}

fn set_dir_value(direction: &Direction, value: i32, tallest: &mut Tallest) -> i32 {
  match direction {
    Left => tallest.left = Some(value),
    Top => tallest.top = Some(value),
    Right => tallest.right = Some(value),
    Bottom => tallest.bottom = Some(value)
  }
  value
}

fn get_dir_value(direction: &Direction, tallest: &Tallest) -> Option<i32> {
  match direction {
    Left => tallest.left,
    Top => tallest.top,
    Right => tallest.right,
    Bottom => tallest.bottom
  }
}

fn get_increments(direction: &Direction) -> (i8, i8) {
  match direction {
    Left => (-1, 0),
    Top => (0, -1),
    Right => (1, 0),
    Bottom => (0, 1)
  }
}

enum Direction {
  Left,
  Top,
  Right,
  Bottom
}

struct Tallest {
  left: Option<i32>,
  top: Option<i32>,
  right: Option<i32>,
  bottom: Option<i32>,
}

impl Default for Tallest {
    fn default() -> Self {
        Self { left: None, top: None, right: None, bottom: None }
    }
}
impl Clone for Tallest {
    fn clone(&self) -> Self {
        Self { left: self.left.clone(), top: self.top.clone(), right: self.right.clone(), bottom: self.bottom.clone() }
    }
}