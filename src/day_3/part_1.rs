use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_1() {
    let file_result = File::open("./data/day_3.txt");
    let file = file_result.unwrap();

    let reader = BufReader::new(file);
    let mut both: Vec<Vec<char>> = Vec::new();

    // go through elf calories
    for linebuf in reader.lines() {
        let line: String = linebuf.unwrap();

        // split into two strings
        let first_part: String = line.chars().take(line.len()/2).collect();
        let second_part: String = line.chars().skip(line.len()/2).take(line.len()-line.len()/2).collect();

        
        // collect list of characters that are the same
        let mut first_part_chars: HashMap<char, u32> = HashMap::new();
        let mut inner_vec: Vec<char> = Vec::new();
        for c in first_part.chars() {
            first_part_chars.insert(c, 1);
        }

        for c in second_part.chars() {
            if first_part_chars.contains_key(&c) {
                inner_vec.push(c);
                break;
            }
        }
        both.push(inner_vec);
    }

    // add it up
    let mut sum: u32 = 0;
    for inner_vec in both.iter() {
        for c in inner_vec.iter() {
            print!("char: {}, value: {}", c, get_value(*c));
            sum += get_value(*c);
        }
        println!();
    }

    println!("Sum: {}", sum);
}


fn get_value(c: char) -> u32 {
    let c_val = c as u32;
    if c_val >= 65 && c_val <= 90 {
        return c_val - 38
    } else if c_val >= 97 && c_val <= 122 {
        return c_val - 96;
    } else {
        panic!("Shouldn't be here");
    }
}
