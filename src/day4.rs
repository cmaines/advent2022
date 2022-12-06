pub fn day_4() {
  find_contains(|assignment_a, assignment_b| assignment_a.start >= assignment_b.start && assignment_a.end <= assignment_b.end);
}

pub fn day_4_part_2() {
  find_contains(|assignment_a, assignment_b| assignment_a.start >= assignment_b.start && assignment_a.start <= assignment_b.end);
}

fn find_contains<F>(contains_func: F)
  where F: Fn(&Assignment, &Assignment) -> bool {
  let mut total: u32 = 0;

  if let Ok(lines) = super::utils::read_lines("input4.txt") {
    let assignments: Vec<String> = lines.filter_map(|line| line.ok()).collect();
    for assignment_pair in assignments {
      let (assignment_a, assignment_b) = get_assignments(assignment_pair.split(",").collect());
      let contained = contains_func(&assignment_a, &assignment_b) || contains_func(&assignment_b, &assignment_a);

      if contained {
        total += 1;
      }
    }
  }

  println!("Total is {}", total);
}

struct Assignment {
  start: u8,
  end: u8
}

fn get_assignments(pair: Vec<&str>) -> (Assignment, Assignment) {
  (get_assignment(pair[0]), get_assignment(pair[1]))
}

fn get_assignment(range: &str) -> Assignment {
  let split_range: Vec<&str> = range.split("-").collect();
  let start: u8 = split_range[0].parse().unwrap();
  let end: u8 = split_range[1].parse().unwrap();
  Assignment { start, end }
}