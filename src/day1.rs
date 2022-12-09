use sorted_vec::SortedVec;

pub fn day_1() {
    let mut counts = SortedVec::new();
    if let Ok(lines) = super::utils::read_lines("input1.txt") {
        let mut curr = 0;
        for line in lines {
            match line.as_deref() {
                Ok("") =>  {
                    counts.insert(curr);
                    curr = 0;
                },
                Ok(str) => curr += str.parse::<u32>().unwrap(),
                _ => panic!("Unknown string")
            }
        }
        counts.insert(curr);
        while counts.len() > 3 {
            counts.remove_index(0);
        }
    }
    
    let total: u32 = counts.iter().sum();
    println!("Calories are {}", total);
}