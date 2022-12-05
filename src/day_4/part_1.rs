use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_1() {
    let file_result = File::open("./data/day_4.txt");
    let file = file_result.unwrap();

    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for linebuf in reader.lines() {
        let line: String = linebuf.unwrap();
        let split = line.split(',');
        let vec: Vec<&str> = split.collect::<Vec<&str>>();

        let range1 = vec[0];
        let range2 = vec[1];

        let range1_1 = range1.split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let range1_2 = range1.split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let range2_1 = range2.split('-').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let range2_2 = range2.split('-').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        // check if range1 in range2
        if (range1_1 >= range2_1 && range1_2 <= range2_2) {
            sum += 1;
        } else if (range2_1 >= range1_1 && range1_2 >= range2_2) {
            sum += 1;
        }
        // println!("range1: {}, range2: {}, sum: {}", range1, range2, sum);
    }

    println!("Sum: {}", sum);
}
