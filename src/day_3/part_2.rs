use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_2() {
    let file_result = File::open("./data/day_3.txt");
    let file = file_result.unwrap();

    let reader = BufReader::new(file);
    let mut same: Vec<Vec<char>> = Vec::new();

    // go through stuff
    let mut i: u32 = 1;
    let mut str1: String = String::from("");
    let mut str2: String = String::from("");
    let mut str3: String = String::from("");
    for linebuf in reader.lines() {
        let line: String = linebuf.unwrap();

        if i % 3 == 2 {
            str1 = line;
            i += 1;
            continue;
        } else if i % 3 == 1 {
            str2 = line;
            i += 1;
            continue;
        } else if i % 3 == 0 {
            str3 = line;
            i += 1;
        }

        // collect list of characters that are the same
        let mut first_part_chars: HashMap<char, u32> = HashMap::new();
        for c in str1.chars() {
            first_part_chars.insert(c, 1);
        }

        let mut second_part_chars: HashMap<char, u32> = HashMap::new();
        for c in str2.chars() {
            if first_part_chars.contains_key(&c) {
                second_part_chars.insert(c, 1);
            }
        }

        let mut inner_vec: Vec<char> = Vec::new();
        for c in str3.chars() {
            if second_part_chars.contains_key(&c) {
                inner_vec.push(c);
                break;
            }
        }
        same.push(inner_vec);

        // clear the strings
        str1 = String::from("");
        str2 = String::from("");
        str3 = String::from("");
    }

    // add it up
    let mut sum: u32 = 0;
    for inner_vec in same.iter() {
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
