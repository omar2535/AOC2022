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

    let len = original.len();

    // generate list for up matrix
    for i in 0..len {
        let mut row: Vec<u32> = vec![original[0][i]];
        let mut max: u32 = row[0];
        for j in 1..len {
            max = std::cmp::max(max, original[j][i]);
            row.push(max.clone());
        }
        up.push(row);
    }

    // generate list for down matrix
    for i in 0..len {
        let mut row: Vec<u32> = vec![original[len-1][i]];
        let mut max: u32 = row[0];
        for j in 2..len {
            max = std::cmp::max(max, original[len-j][i]);
            row.push(max.clone());
        }
    }
}

// 1 2 3 4 5   1 1 1 1 1
// 1 2 3 4 5   2 2 2 2 2
// 1 2 3 4 5 > 3 3 3 3 3
// 1 2 3 4 5   4 4 4 4 4
// 1 2 3 4 5   5 5 5 5 5
// must be a square matrix
pub fn transpose_matrix(input_vec: &mut Vec<Vec<u32>>) {
    let len: usize = input_vec.len();

    for i in 0..len {
        for j in 0..len {
            let temp = input_vec[i][j];
            input_vec[i][j] = input_vec[j][i];
            input_vec[j][i] = temp;
        }
    }
}
