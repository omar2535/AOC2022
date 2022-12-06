use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn part_1() {
    let file_result = File::open("./data/day_5.txt");
    let start_line: usize = 10;
    let file = file_result.unwrap();
    let reader = BufReader::new(file);

    let crate1: Vec<char> = vec!['Q', 'M', 'G', 'C', 'L'];
    let crate2: Vec<char> = vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'];
    let crate3: Vec<char> = vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'];
    let crate4: Vec<char> = vec!['J', 'F', 'D', 'V', 'Q', 'P'];
    let crate5: Vec<char> = vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'];
    let crate6: Vec<char> = vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'];
    let crate7: Vec<char> = vec!['H', 'C', 'T'];
    let crate8: Vec<char> = vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'];
    let crate9: Vec<char> = vec!['Z', 'F', 'H', 'G'];
    
    let mut crates: Vec<Vec<char>> = vec![
        Vec::new(), crate1, crate2, crate3, crate4, crate5, crate6, crate7, crate8, crate9
    ];
    
    for (index, linebuf) in reader.lines().enumerate() {
        // skip the first N lines
        if index < start_line { continue }

        let line: String = linebuf.unwrap();
        let split_line: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
        let amount: u32 = split_line[1].parse::<u32>().unwrap();
        let from: usize = split_line[3].parse::<usize>().unwrap();
        let to: usize = split_line[5].parse::<usize>().unwrap();

        // println!("Amount: {}, from: {}, dest: {}", amount, from, to);
        
        // move from crates[from] to crates[to] the amount
        for _ in 0..amount {
            let temp: char = crates[from].pop().unwrap();
            crates[to].push(temp);
        }
    }
    
    // print top of each crate
    for i in 1..crates.len() {
        print!("{}", crates[i].pop().unwrap());
    }
    println!("");
}
