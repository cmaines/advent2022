use State::*;

const X_OFFSET: usize = 330; // 431;
const Y_OFFSET: usize = 0;
const NUM_ROWS: usize = 160;
const NUM_COLS: usize = 330;
const SAND_X: usize = 500;
const SAND_Y: usize = 0;

pub fn day_14() {
  let mut graph = execute();

  let lowest_level = NUM_ROWS - graph.iter().rev().position(|row| row.contains(&Rock)).unwrap() + Y_OFFSET;
  let successful_drops = count_successful_drops(&mut graph, lowest_level);
  print_graph(&graph);
  println!("Successful drops: {}", successful_drops);
}

pub fn day_14_part_2() {
  let mut graph = execute();

  let lowest_level = 2 + NUM_ROWS - graph.iter().rev().position(|row| row.contains(&Rock)).unwrap() + Y_OFFSET;
  graph[lowest_level - 1] = [Rock; NUM_COLS];

  let successful_drops = count_successful_drops(&mut graph, lowest_level);
  print_graph(&graph);
  println!("Successful drops: {}", successful_drops);
}

fn execute() -> [[State; NUM_COLS]; NUM_ROWS] {
  let mut graph: [[State; NUM_COLS]; NUM_ROWS] = [[Air; NUM_COLS]; NUM_ROWS];
  
  if let Ok(lines) = super::utils::read_lines("input14.txt") {
    for line in lines.flatten() {
      let mut coords = get_coords(line);
      process_coords(&mut graph, &mut coords);
    }
  }

  graph
}

fn print_graph(graph: &[[State; NUM_COLS]; NUM_ROWS]) {
  for row in graph {
    for state in row {
      let c = match state {
        Air => '.',
        Rock => '#',
        Sand => 'o'
      };
      print!("{}", c);
    }
    println!();
  }
}

fn count_successful_drops(graph: &mut [[State; NUM_COLS]; NUM_ROWS], lowest_level: usize) -> u32 {
  let mut successful_drops = 0;
  while drop_sand(graph, lowest_level) {
    successful_drops += 1;
  }

  successful_drops
}

fn drop_sand(graph: &mut [[State; NUM_COLS]; NUM_ROWS], lowest_level: usize) -> bool {
  let (mut x, mut y) = (SAND_X - X_OFFSET, SAND_Y - Y_OFFSET);
  if graph[y][x] == Sand {
    return false;
  }

  while y != lowest_level {
    if graph[y+1][x] == Air {
      y += 1;
    } else if x != 0 && graph[y+1][x-1] == Air {
      y += 1;
      x -= 1;
    } else if x+1 != NUM_COLS && graph[y+1][x+1] == Air {
      y += 1;
      x += 1;
    } else {
      graph[y][x] = Sand;
      return true;
    }
  }

  return false;
}

fn process_coords(graph: &mut [[State; NUM_COLS]; NUM_ROWS], coords: &mut Vec<(i32, i32)>) {
  let (mut curr_x, mut curr_y) = coords.remove(0);
  while !coords.is_empty() {
    let (target_x, target_y) = coords.remove(0);
    let gradient_x = (target_x - curr_x).signum();
    let gradient_y = (target_y - curr_y).signum();

    while curr_x != target_x || curr_y != target_y {
      graph[curr_y as usize - Y_OFFSET][curr_x as usize - X_OFFSET] = Rock;
      curr_x += gradient_x;
      curr_y += gradient_y;
    }
    graph[curr_y as usize - Y_OFFSET][curr_x as usize - X_OFFSET] = Rock;
  }
}

fn get_coords(line: String) -> Vec<(i32, i32)> {
  let instructions: Vec<&str> = line.split(" -> ").collect();
  let mut coordinates = vec![];
  for instruction in instructions.into_iter() {
    let coords: Vec<&str> = instruction.split(',').collect();
    let Some(f) = coords.get(0) else { panic!() };
    let x: i32 = coords.get(0).unwrap().parse().unwrap();
    let y: i32 = coords.get(1).unwrap().parse().unwrap();
    coordinates.push((x, y));
  }
  return coordinates;
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
  Air,
  Rock,
  Sand
}