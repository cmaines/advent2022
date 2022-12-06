use sorted_vec::SortedVec;

pub fn day_1() {
    let mut counts = SortedVec::new();
    
    if let Ok(lines) = read_lines("input1.txt") {
        let mut curr = 0;
        for line in lines {
            if let Ok(str) = line {
                if str == "" {
                    counts.insert(curr);
                    curr = 0;
                } else if let Ok(num) = str.parse::<u32>() {
                    curr += num;
                }
            }
        }
        counts.insert(curr);
        while counts.len() > 3 {
            counts.remove_index(0);
        }
    }
    
    let mut total = 0;
    for num in counts.iter() {
        total += num;
    }
    println!("Calories are {:?}", total);
}