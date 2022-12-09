use std::{collections::HashSet, slice::Windows};

fn all_chars_different(window: &&[char]) -> bool {
    let mut set = HashSet::new();
    for c in window.iter() {
        set.insert(c);
    }
    set.len() == window.len()
}

pub fn main_day6() {
    println!("----- DAY 6 --------");
    let test_data = std::fs::read_to_string("./data/day6.txt").unwrap();

    let chars: Vec<char> = test_data.chars().collect();

    let sop_marker: String = chars.windows(4).find(all_chars_different).unwrap().iter().collect();
    let pos = test_data.find(&sop_marker).unwrap() + 4;
    println!("SOP Marker {:?} at {}", sop_marker, pos);

    let som_marker: String = chars.windows(14).find(all_chars_different).unwrap().iter().collect();
    let pos = test_data.find(&som_marker).unwrap() + 14;
    println!("SOM Marker {:?} at {}", som_marker, pos);
}
