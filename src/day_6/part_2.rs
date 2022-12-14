use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn day_2() {
    let file_result = File::open("./data/day_6.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);
    let mut line: String = String::from("");

    for linebuf in reader.lines() {
        line = linebuf.unwrap();
    }

    let mut map: HashMap<char, u32> = HashMap::new();
    let mut last_letter: char;
    let mut num_inside = 0;
    let matching: usize = 14;
    for (index, c) in line.chars().enumerate() {
        if num_inside < matching {
            if map.contains_key(&c) {
                *map.get_mut(&c).unwrap() += 1;
            } else {
                map.insert(c, 1);
            }
            num_inside += 1;
            continue;
        }
        last_letter = line.chars().nth(index-matching).unwrap();

        print!("Num keys: {} | ", &map.keys().len());
        print!("Last letter: {} |", last_letter);
        print_map(&map);
        println!();

        
        if map.keys().len() >= matching {
            println!("Found!: {}", index);
            break;
        }

        // update next letter value
        if map.contains_key(&c) {
            *map.get_mut(&c).unwrap() += 1;
        } else {
            map.insert(c, 1);
        }
        
        // update last letter value
        if *map.get(&last_letter).unwrap() > 1 {
            *map.get_mut(&last_letter).unwrap() -= 1;
        } else {
            println!("Removing");
            map.remove(&last_letter);
        }
    }
}

fn print_map(map: &HashMap<char, u32>) {
    for (k, v) in map.iter() {
        print!("{}: {}, ", k, v);
    }
}
