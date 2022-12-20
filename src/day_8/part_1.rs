// day 8
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Ideal solution:
// create a 4 differnet max mtrixes for each direction.
// This uses O(n) space and O(n) lookup
// but I'll jus code the simple solution since we have a static file

pub fn part_1() {
    let file_result = File::open("./data/day_8.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);

    // our matrices
    let mut original: Vec<Vec<u32>> = Vec::new();
    
    // create the original matrix
    for (index, linebuf) in reader.lines().enumerate() {
        let mut row: Vec<u32> = vec![];
        for digit_str in linebuf.unwrap().chars() {
            row.push(digit_str.to_digit(10).unwrap());
        }
        original.push(row);
    }

    // square matrix, so we just use len for both
    let len = original.len();
    let mut total = 0;

    for y in 0..len {
        for x in 0..len {
            if y == 0 || x == 0 || y == len-1 || x == len-1 {
                // at the border
                total += 1;
            } else {
                // current tree height
                let cur_tree: u32 = original[y][x];

                // check each direction max
                let vertical: Vec<u32> = get_vertical_vector_at_x(&original, x);
                let horizontal: &Vec<u32> = get_horizontal_vector_at_y(&original, y);
                
                let up: &Vec<u32> = &vertical[0..y].to_vec();
                let down: &Vec<u32> = &vertical[y+1..vertical.len()].to_vec();
                let left: &Vec<u32> = &horizontal[0..x].to_vec();
                let right: &Vec<u32> = &horizontal[x+1..horizontal.len()].to_vec();

                let up_max = up.iter().max().unwrap();
                let down_max = down.iter().max().unwrap();
                let left_max = left.iter().max().unwrap();
                let right_max = right.iter().max().unwrap();

                if cur_tree > *up_max || cur_tree > *down_max || 
                   cur_tree > *left_max || cur_tree > *right_max {
                    total += 1;
                }                
            }
        }
    }
    println!("Num trees: {}", total);
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

// Gets the entire horizontal row as a vector
pub fn get_horizontal_vector_at_y(input_vec: &Vec<Vec<u32>>, y: usize) -> &Vec<u32> {
    return &input_vec[y];
}

// gets the entire vertical row as a vector
pub fn get_vertical_vector_at_x(input_vec: &Vec<Vec<u32>>, x: usize) -> Vec<u32> {
    let len: usize = input_vec.len();
    let mut y_vec: Vec<u32> = vec![];

    for y in 0..len {
        y_vec.push(input_vec[y][x]);
    }

    return y_vec;
}
