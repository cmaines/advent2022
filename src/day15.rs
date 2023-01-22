use regex::{Regex, Captures};

const LOWER_BOUND: i32 = 0;
const UPPER_BOUND: i32 = 4_000_000;

pub fn day_15() {
    let sensors = execute();
    let no_beacon_positions = count_no_beacons(&sensors, 2000000);
    println!("Result is {}", no_beacon_positions);
}

pub fn day_15_part_2() {
    let sensors = execute();
    for row in LOWER_BOUND..UPPER_BOUND {
        let Some(missing_pos) = find_missing_beacon(&sensors, row) else { continue };
        let frequency: u64 = (4_000_000 * missing_pos as u64) + row as u64;
        println!("Result is {}", frequency);
        break;
    }
}

fn execute() -> Vec<Sensor> {
    let re = Regex::new(r"^Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)$").unwrap();
    let mut sensors: Vec<Sensor> = vec![];

    if let Ok(lines) = super::utils::read_lines("input15.txt") {
      for line in lines.flatten() {
        if let Some(capture) = re.captures(line.as_str()) {
            let sensor_x = parse_num(&capture, "s_x").unwrap();
            let sensor_y = parse_num(&capture, "s_y").unwrap();
            let beacon_x = parse_num(&capture, "b_x").unwrap();
            let beacon_y = parse_num(&capture, "b_y").unwrap();
            sensors.push(Sensor { sensor_x, sensor_y, beacon_x, beacon_y });
        }
      }
    }
  
    sensors
}

fn count_no_beacons(sensors: &Vec<Sensor>, row: i32) -> usize {
    let no_beacon_pos = get_reduced_ranges(sensors, row);
    let mut no_beacon_count = 0;
    for range in no_beacon_pos {
        no_beacon_count += range.1 - range.0;
    }
    no_beacon_count as usize
}

fn find_missing_beacon(sensors: &Vec<Sensor>, row: i32) -> Option<i32> {
    let no_beacon_pos = get_reduced_ranges(sensors, row);
    let missing_value = no_beacon_pos.get(0)?.1;
    if missing_value < UPPER_BOUND {
        return Some(missing_value + 1);
    }
    None    
}

fn get_reduced_ranges(sensors: &Vec<Sensor>, row: i32) -> Vec<(i32, i32)> {
    let mut no_beacon_pos: Vec<(i32,i32)> = vec![];
    for sensor in sensors {
        let Some((start_x, end_x)) = get_row_range(sensor, row) else { continue };
        no_beacon_pos.push((start_x, end_x));
    }
    
    no_beacon_pos.sort_by_key(|(start, _)| *start);
    let mut no_beacon_pos_reduced = vec![];
    let mut curr_range = no_beacon_pos.remove(0);
    for pos in no_beacon_pos {
        if pos.0 <= curr_range.1 + 1 {
            if pos.1 > curr_range.1 {
                curr_range.1 = pos.1;
            }
        } else {
            no_beacon_pos_reduced.push(curr_range);
            curr_range = pos;
        }
    }
    no_beacon_pos_reduced.push(curr_range);

    no_beacon_pos_reduced
}

fn get_row_range(sensor: &Sensor, row: i32) -> Option<(i32, i32)> {
    let avail_distance = sensor.get_distance() as i32 - (sensor.sensor_y - row).abs();
    if avail_distance < 0 {
        return None;
    }
    let start_x = sensor.sensor_x - avail_distance;
    let end_x = sensor.sensor_x + avail_distance;
    Some((start_x, end_x))
}

fn parse_num(capture: &Captures, name: &str) -> Option<i32> {
    capture.name(name)?.as_str().parse().ok()
}

struct Sensor {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32
}

impl Sensor {
    fn get_distance(&self) -> u32 {
        ((self.sensor_x - self.beacon_x).abs() + (self.sensor_y - self.beacon_y).abs()) as u32
    }
}