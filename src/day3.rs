use std::collections::{HashSet, btree_set::Intersection};

pub fn day_3() {
    if let Ok(lines) = super::utils::read_lines("input3.txt") {

        let mut total = 0;
        let mut set_a = HashSet::new();
        let mut set_b = HashSet::new();
        
        for line in lines {
            if let Ok(sack) = line {
                let (compartment_a, compartment_b) = sack.split_at(sack.len() / 2);

                set_a.extend(compartment_a.chars());
                set_b.extend(compartment_b.chars());
   
                let intersection = set_a.intersection(&set_b).next().unwrap();
                total += get_priority(intersection);

                set_a.clear();
                set_b.clear();
            }
        }
        println!("Total is {}", total);
    }
}

pub fn day_3_part_2() {
    if let Ok(lines) = super::utils::read_lines("input3.txt") {

        let mut total = 0;
        let mut set_a = HashSet::new();
        let mut set_b = HashSet::new();
        let mut set_c = HashSet::new();
        let sacks: Vec<String> = lines.filter_map(|line| line.ok()).collect();
        
        for i in (0..sacks.len()).step_by(3) {
            set_a.extend(sacks.get(i).unwrap().chars());
            set_b.extend(sacks.get(i+1).unwrap().chars());
            set_c.extend(sacks.get(i+2).unwrap().chars());

            set_a.retain(|item| set_b.contains(item) && set_c.contains(item));
            total += get_priority(set_a.iter().next().unwrap());

            set_a.clear();
            set_b.clear();
            set_c.clear();
        }
        
        println!("Total is {}", total);
    }
}

fn get_priority(intersection: &char) -> u32 {
    let char_code = *intersection as u32;
    char_code - (if intersection.is_ascii_uppercase() { 38 } else { 96 })
}