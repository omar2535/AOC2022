// day 8
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Ideal solution:
// create a 4 differnet max mtrixes for each direction.
// This uses O(n) space and O(n) lookup
// but I'll jus code the simple solution since we have a static file

pub fn part_2() {
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
    let mut max_scenic_score = 0;

    for y in 0..len {
        for x in 0..len {
            // current tree height
            let cur_tree: u32 = original[y][x];
            println!("For tree: '{}' at x: {}, y: {}", cur_tree, x, y);

            // check each direction max
            let vertical: Vec<u32> = get_vertical_vector_at_x(&original, x);
            let horizontal: &Vec<u32> = get_horizontal_vector_at_y(&original, y);
            
            let up: &Vec<u32> = &vertical[0..y].to_vec().into_iter().rev().collect();
            let down: &Vec<u32> = &vertical[y+1..vertical.len()].to_vec();
            let left: &Vec<u32> = &horizontal[0..x].to_vec().into_iter().rev().collect();
            let right: &Vec<u32> = &horizontal[x+1..horizontal.len()].to_vec();

            let up_count = get_distance_until_blocked_view(&cur_tree, up);
            let down_count = get_distance_until_blocked_view(&cur_tree, down);
            let left_count = get_distance_until_blocked_view(&cur_tree, left);
            let right_count = get_distance_until_blocked_view(&cur_tree, right);

            let scenic_score: u32 = up_count * down_count * left_count * right_count;

            if scenic_score > max_scenic_score {
                print!("Max reached!, figures: {}, {}, {}, {} | ", up_count, down_count, left_count, right_count);
                max_scenic_score = scenic_score;
            }
            
        }
    }
    println!("Max scenic score: {}", max_scenic_score);
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

// get distance until view is blocked
pub fn get_distance_until_blocked_view(orig_tree_height: &u32, tree_heghts_vec: &Vec<u32>) -> u32 {
    println!("{:?}", tree_heghts_vec);
    let mut count: u32 = 0;
    for tree in tree_heghts_vec.iter() {
        count += 1;
        if tree >= orig_tree_height {
            break;
        }
    }
    return count;
}