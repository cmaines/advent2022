use std::collections::HashSet;

pub fn day_12() {
  execute(true,
     'S', 
     'E', 
     find_steps_to_end,
     |neighbor_height, node_height| neighbor_height <= node_height + 1
  );
}

pub fn day_12_part_2() {
  execute(false, 
    'E', 
    'S', 
    find_shortest_low_point,
    |neighbor_height, node_height| neighbor_height + 1 >= node_height
  );
}

fn execute(exit_early: bool, start_char: char, end_char: char, end_func: EndFn, height_func: HeightFn) {
  let mut nodes = vec![];
  let mut unvisited = HashSet::new();
  let mut end: (usize, usize) = (0, 0);

  if let Ok(lines) = super::utils::read_lines("input12.txt") {
    let mut rownum = 0;
    for line in lines {
      if let Ok(line) = line {
        let line = line.chars();
        let mut row = vec![];
        let mut colnum = 0;
        for character in line {
          let height = get_char_height(character);
          let dist = if character == start_char { 0 } else { u32::MAX };
          if character == end_char {
            end = (colnum, rownum);
          }
          row.push(Node { x: colnum, y: rownum, height, prev: Option::None, dist, visited: false });
          unvisited.insert((colnum, rownum));
          colnum += 1;
        }
        nodes.push(row);
        rownum += 1;
      }
    }

    while !unvisited.is_empty() {
      let node = get_lowest_unvisited(&nodes);
      let x = node.x;
      let y = node.y;
      let dist = node.dist;
      let height = node.height;
      let coord = (x, y);

      nodes[y][x].visited = true;
      unvisited.remove(&coord);
      
      check_and_update(x.checked_sub(1), Option::Some(y), (x, y), dist, height, &mut nodes, height_func);
      check_and_update(Option::Some(x+1), Option::Some(y), (x, y), dist, height, &mut nodes, height_func);
      check_and_update(Option::Some(x), y.checked_sub(1), (x, y), dist, height, &mut nodes, height_func);
      check_and_update(Option::Some(x), Option::Some(y+1), (x, y), dist, height, &mut nodes, height_func);

      if exit_early && nodes[end.1][end.0].dist != u32::MAX {
        println!("Found the end!");
        break;
      }
    }

    end_func(end, &nodes);
  }
}

type EndFn = fn((usize, usize), &Vec<Vec<Node>>) -> Option<u32>;
type HeightFn = fn(u32, u32) -> bool;

fn find_steps_to_end(end: (usize, usize), nodes: &Vec<Vec<Node>>) -> Option<u32> {
  let mut steps = 0;
  let mut node = &nodes[end.1][end.0];
  while node.dist != 0 {
    steps += 1;
    if node.prev.is_none() {
      return Option::None;
    }
    let prev = node.prev.unwrap();
    node = &nodes[prev.1][prev.0];
  }
  println!("Steps to end: {}", steps);
  Option::Some(steps)
}

fn find_shortest_low_point(end: (usize, usize), nodes: &Vec<Vec<Node>>) -> Option<u32> {
  let mut shortest_steps = u32::MAX;
  for row in nodes {
    for end_node in row {
      if end_node.height != 1 {
        continue;
      }

      let steps = find_steps_to_end((end_node.x, end_node.y), nodes);
      if steps.is_none() {
        continue;
      }
      let steps = steps.unwrap();
      shortest_steps = if steps < shortest_steps { steps } else { shortest_steps };
    }
  }
  println!("Shortest steps to end: {}", shortest_steps);
  Option::Some(shortest_steps)
}


fn check_and_update(x: Option<usize>, y: Option<usize>, coords: (usize, usize), dist: u32, height: u32, nodes: &mut Vec<Vec<Node>>, height_func: HeightFn) {
  if x.is_none() || y.is_none() || dist == u32::MAX {
    return;
  }
  let x = x.unwrap();
  let y = y.unwrap();

  if !can_go(x, y, height, nodes, height_func) {
    return;
  }

  let newdist = dist + 1;
  update_node(&mut nodes[y][x], newdist, coords);
}

fn update_node(node: &mut Node, dist: u32, prev: (usize, usize)) {
  if dist < node.dist {
    node.dist = dist;
    node.prev = Some(prev);
  }
}

fn get_lowest_unvisited(nodes: &Vec<Vec<Node>>) -> &Node {
  let mut lowest_dist = u32::MAX;
  let mut lowest_node= &nodes[0][0];

  for row in nodes {
    for node in row {
      if node.dist <= lowest_dist && !node.visited {
        lowest_node = node;
        lowest_dist = node.dist;
      }
    }
  }

  lowest_node
}

fn can_go(x: usize, y: usize, height: u32, nodes: &Vec<Vec<Node>>, height_fn: fn(u32, u32) -> bool) -> bool {
  if x >= nodes[0].len() || y >= nodes.len() {
    return false;
  }

  return height_fn(nodes[y][x].height, height);
}

fn get_char_height(character: char) -> u32 {
  if character == 'S' {
    return 1;
  }
  if character == 'E' {
    return 26
  }
  (character as u32) - 96 as u32
}

struct Node {
  x: usize,
  y: usize,
  height: u32,
  prev: Option<(usize, usize)>,
  dist: u32,
  visited: bool
}