// day 8
use std::fs::File;
use std::io::{prelude::*, BufReader};


pub fn part_1() {
    let file_result = File::open("./data/day_8.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);

    // our matrices
    let mut original: Vec<Vec<u32>> = Vec::new();
    let mut up: Vec<Vec<u32>> = Vec::new();
    let mut down: Vec<Vec<u32>> = Vec::new();
    let mut left: Vec<Vec<u32>> = Vec::new();
    let mut right: Vec<Vec<u32>> = Vec::new();
    
    // create the original matrix
    for (index, linebuf) in reader.lines().enumerate() {
        let mut row: Vec<u32> = vec![];
        for digit_str in linebuf.unwrap().chars() {
            row.push(digit_str.to_digit(10).unwrap());
        }
        original.push(row);
    }
}
