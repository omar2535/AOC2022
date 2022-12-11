// day 8
use std::fs::File;
use std::io::{prelude::*, BufReader};


pub fn part_1() {
    let file_result = File::open("./data/day_8.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);

    // our matrices
    let original: Vec<Vec<u8>> = Vec::new();
    let up: Vec<Vec<u8>> = Vec::new();
    let down: Vec<Vec<u8>> = Vec::new();
    let left: Vec<Vec<u8>> = Vec::new();
    let right: Vec<Vec<u8>> = Vec::new();

    for (index, linebuf) in reader.lines().enumerate() {
        for digit_str in linebuf.unwrap().chars() {
            println!("{}", digit_str);
        }
    }
}
